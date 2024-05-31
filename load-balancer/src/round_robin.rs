use crate::server::Server;

pub struct RoundRobin {
    servers: Vec<Server>,
    current_index: usize,
}

impl RoundRobin {
    pub fn new(servers: Vec<Server>) -> Self {
        RoundRobin {
            servers,
            current_index: 0,
        }
    }

    pub fn run() {
        let servers = Vec::from([
            Server::new("http://localhost:8080", 1),
            Server::new("http://localhost:8081", 2),
            Server::new("http://localhost:8082", 3),
        ]);

        let mut round_robin = RoundRobin::new(servers);
        for _ in 0..10 {
            let server = round_robin.get_server();
            // Do something with the server
            println!("Using server: {:?}", server);
        }
    }
}

impl RoundRobin {
    fn get_server(&mut self) -> Server {
        let retries = self.servers.len();
        let mut tries = 0;
        loop {
            let server = &self.servers[self.current_index];
            self.current_index = (self.current_index + 1) % self.servers.len();

            if server.active {
                return server.clone();
            }

            tries += 1;
            if tries >= retries {
                panic!("No active servers available");
            }
        }
    }
}
