use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use std::cell::RefCell;
use std::ops::Deref;

use life::board::evolve_board;
use life::board::Board;

pub struct ThreadPool {
    threads: usize,
    master_rx: Receiver<RefCell<Board>>,
    workers: Vec<Sender<RefCell<Board>>>,

}

impl ThreadPool {
    pub fn dispatch_threads(&self,alpha: &RefCell<Board>) -> () {
        for i in 0usize..self.threads {
            match self.workers[i].send(alpha.clone()) {
                Ok(_) => (),
                Err(e) => println!("Thread {}: Sending work failed {}",i,e),
            };
        }
    }

    pub fn compose_threads(&self,beta: &RefCell<Board>) -> () {
        {
            //get mut access to beta
            let mut beta_mut = beta.borrow_mut();
            beta_mut.board.clear();

            for _ in 0usize..self.threads {
                let a1 = self.master_rx.recv().unwrap();
                let a2 = a1.borrow();
                let ref a3 = a2.deref().board;
                beta_mut.board.union_with(a3);
            }
        }
    }
}

pub fn init_threads(threads:usize, input: &Board) -> ThreadPool {
    //create master channel
    let (master_tx,master_rx): (Sender<RefCell<Board>>,Receiver<RefCell<Board>>) = mpsc::channel();

    //Store the workers channel Tx
    let mut workers = Vec::new();

    //calculate the cells to be processed by each thread
    let work_range = input.total / threads;

    let width = input.width.clone();
    let height = input.height.clone();

    //build threads
    for i in 0usize..threads {
        let (thread_tx,thread_rx): (Sender<RefCell<Board>>,Receiver<RefCell<Board>>) = mpsc::channel();
        workers.push(thread_tx);
        let master_tx = master_tx.clone();
        thread::spawn(move|| {
            let id = i;
            let (start,end) = (i*work_range,(i*work_range)+work_range);
            //local working space
            let c = Board::new(width,height);
            let charlie = &mut RefCell::new(c);

            //process loop
            loop {
                //get a new job
                match thread_rx.recv() {
                    Ok(x) => {
                        //println!("Thread {} Got work",id);
                        {
                            evolve_board(charlie, &x,start,end);
                        }

                        match master_tx.send(charlie.clone()) {
                            Ok(_) => (),
                            Err(e) => println!("Thread {}: Returning work failed {}",id,e),
                        }
                        ()},
                    Err(e) => {println!("Thread {}: Got an Err dying.. {}",id,e);break}, //end the thread
                };
            }
        });
    }
    ThreadPool{threads:threads,master_rx:master_rx,workers:workers}
}
