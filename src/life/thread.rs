use std::collections::{BTreeSet,BitvSet};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::Thread;

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use life::board::evolve_board;

pub struct thread_pool {
    threads: usize,
    masterRx: Receiver<RefCell<BitvSet>>,
    workers: Vec<Sender<RefCell<BitvSet>>>,

}

impl thread_pool {
    pub fn dispatch_threads(&self,alpha: &RefCell<BitvSet>) -> () {
        for i in 0us..self.threads {
            self.workers[i].send(alpha.clone());
        }
    }

    pub fn compose_threads(&self,beta: &RefCell<BitvSet>) -> () {
        {
            //get mut access to beta
            let mut beta_mut = beta.borrow_mut();
            beta_mut.clear();

            for i in 0us..self.threads {
                beta_mut.deref_mut().union_with(self.masterRx.recv().unwrap().borrow().deref());
            }
        }
    }
}

pub fn init_threads(threads:usize ,work_total:usize,width:usize,height:usize) -> thread_pool {
    //create master channel
    let (masterTx,masterRx): (Sender<RefCell<BitvSet>>,Receiver<RefCell<BitvSet>>) = mpsc::channel();

    //Store the workers channel Tx
    let mut workers = Vec::new();

    //calculate the cells to be processed by each thread
    let work_range = work_total / threads;

    //build threads
    for i in 0us..threads {
        let (threadTx,threadRx): (Sender<RefCell<BitvSet>>,Receiver<RefCell<BitvSet>>) = mpsc::channel();
        workers.push(threadTx);
        let mTx = masterTx.clone();
        Thread::spawn(move|| {
            let id = i;
            let (start,end) = (i*work_range,(i*work_range)+work_range);
            //local working space
            let mut c = BitvSet::with_capacity(work_total);
            let charlie = &mut RefCell::new(c);

            //process loop
            loop {
                //get a new job
                match threadRx.recv() {
                    Ok(x) => {
                        println!("Thread {} Got work",id);
                        {
                            evolve_board(charlie.borrow_mut().deref_mut(), x.borrow().deref(),start,end,width,height);
                        }
                        mTx.send(charlie.clone());
                        ()},
                    Err(e) => {println!("{} Got an Err dying.. {}",id,e);break}, //end the thread
                }
            }
        });
    }
    thread_pool{threads:threads,masterRx:masterRx,workers:workers}
}
