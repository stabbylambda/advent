use std::collections::VecDeque;

use intcode::Intcode;

fn main() {
    let input = include_str!("../input.txt");
    let input = Intcode::parse(input);

    let answer = problem1(&input);
    println!("problem 1 answer: {answer}");

    let answer = problem2(&input);
    println!("problem 2 answer: {answer}");
}

type Input = Intcode;

type Packet = (usize, i64, i64);

struct Controller {
    program: Intcode,
    send_queue: VecDeque<Packet>,
}

enum ControllerResult {
    Idle,
    ReceivedPackets,
    SentPacket(Packet),
}

impl Controller {
    fn new(address: usize, program: Intcode) -> Self {
        let mut program = program;
        program.input.push(address as i64);
        program.execute();

        Controller {
            program,
            send_queue: VecDeque::new(),
        }
    }

    fn send_packet(&mut self, x: i64, y: i64) {
        // gotta push backwards
        self.program.input.push(y);
        self.program.input.push(x);
    }

    fn run(&mut self) -> ControllerResult {
        // if we have packets waiting to be sent, just sent the next packet
        if let Some(packet) = self.send_queue.pop_front() {
            return ControllerResult::SentPacket(packet);
        }

        let queue_was_empty = self.program.input.is_empty();

        if queue_was_empty {
            self.program.input.push(-1);
        }

        // run the intcode
        self.program.execute();

        // queue up the packets to send
        for chunk in self.program.output.chunks_exact(3) {
            let packet = (chunk[0] as usize, chunk[1], chunk[2]);
            self.send_queue.push_back(packet);
        }
        // and wipe the output so we don't double send
        self.program.output.clear();

        // send a packet immediately if we've got it
        if let Some(packet) = self.send_queue.pop_front() {
            ControllerResult::SentPacket(packet)
        } else if queue_was_empty {
            ControllerResult::Idle
        } else {
            ControllerResult::ReceivedPackets
        }
    }
}

struct Network {
    controllers: Vec<Controller>,
    nat: Option<(i64, i64)>,
}

enum NetworkResult {
    Running,
    AllIdle,
}

impl Network {
    fn new(input: &Input) -> Self {
        let controllers: Vec<Controller> =
            (0..50).map(|x| Controller::new(x, input.clone())).collect();

        Self {
            controllers,
            nat: None,
        }
    }

    fn run(&mut self) -> NetworkResult {
        let mut result = NetworkResult::AllIdle;
        for n in 0..self.controllers.len() {
            if let Some(controller) = self.controllers.get_mut(n) {
                match controller.run() {
                    ControllerResult::SentPacket((address, x, y)) => {
                        if let Some(receiver) = self.controllers.get_mut(address) {
                            receiver.send_packet(x, y);
                        } else if address == 255 {
                            self.nat = Some((x, y));
                        }
                        result = NetworkResult::Running;
                    }
                    ControllerResult::ReceivedPackets => {
                        result = NetworkResult::Running;
                    }
                    ControllerResult::Idle => (),
                };
            }
        }

        result
    }
}

fn problem1(input: &Input) -> i64 {
    let mut network = Network::new(input);
    loop {
        network.run();
        // bail the first time we see something sent to address 255
        if let Some((_x, y)) = network.nat {
            return y;
        }
    }
}

fn problem2(input: &Input) -> i64 {
    let mut last_y = None;
    let mut network = Network::new(input);
    loop {
        if let NetworkResult::AllIdle = network.run() {
            if let Some((x, y)) = network.nat {
                if let Some(last_y) = last_y {
                    if y == last_y {
                        // bail when we see the same y value twice in a row
                        return y;
                    }
                }

                network.controllers[0].send_packet(x, y);
                last_y = Some(y);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use intcode::Intcode;

    use crate::{problem1, problem2};
    #[test]
    fn first() {
        let input = include_str!("../input.txt");
        let input = Intcode::parse(input);
        let result = problem1(&input);
        assert_eq!(result, 17740)
    }

    #[test]
    fn second() {
        let input = include_str!("../input.txt");
        let input = Intcode::parse(input);
        let result = problem2(&input);
        assert_eq!(result, 12567)
    }
}
