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
            let beta_mut = beta.borrow_mut();
            beta_mut.clear();

            for i in 0us..self.threads {
        println!("render");
                beta_mut.deref_mut().union_with(self.masterRx.recv().unwrap().borrow().deref());
            }
        }
    }
}

pub fn init_threads(usize: threads , usize:work_total) -> thread_pool {
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
            let mut c = BitvSet::with_capacity(total);
            let charlie = &mut RefCell::new(c);
            
            //process loop 
            loop {
                //get a new job
                match threadRx.recv() {
                    Ok(x) => {
                        println!("Thread {} Got work",id);
                        {
                            evolve_board(charlie.borrow_mut().deref_mut(), x.borrow().deref(),start,end);
                        }
                        mTx.send(charlie.clone());
                        ()},
                    Err(_) => {println!("{} Got an Err dying..",id);break}, //end the thread 
                }
            }
        });

        thread_pool{threads:threads,masterRx:masterRx,workers:workers}
    }
}