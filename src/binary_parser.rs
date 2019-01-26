

pub struct BinaryParser {
    buffer: Vec<u8>,
    next: usize,
}

impl BinaryParser {
    pub fn new(buffer: Vec<u8>) -> BinaryParser {
        BinaryParser {
            buffer,
            next: 0,
        }
    }

    pub fn next(&mut self) -> u8 {
        let x = self.buffer[self.next];
        self.next += 1;
        return x;
    }

    pub fn drop(&mut self, n: usize) {
        self.next += n;
    }

    pub fn expect(&mut self, i: u8) {
        let a = self.next();
        if a != i {
            panic!("Expected {}, got {}", i, a);
        }
    }

    pub fn expect_many(&mut self, ls: Vec<u8>) {
        for i in ls {
            let a = self.next();
            if a != i {
                panic!("Expected {}, got {}", i, a);
            }
        }
    }

    pub fn take(&mut self, n: usize) -> Vec<u8> {
        let x = self.buffer[self.next..self.next + n].to_vec();
        self.next += n;
        return x;
    }

    pub fn take_until(&mut self, x: u8) -> Vec<u8> {
        let mut acc = Vec::new();
        let mut i = 0;
        loop {
            let p = self.peek(1)[0];
            if p == x {
                break;
            }

            let n = self.next();
            acc.push(n);
        }

        return acc;
    }

    pub fn peek(&self, n: usize) -> Vec<u8> {
        self.buffer[self.next..self.next + n].to_vec()
    }

    pub fn current_location(&self) -> usize {
        self.next
    }

    pub fn seek_to(&mut self, i: usize) {
        self.next = i;
    }

    pub fn is_it_the_end(&mut self) -> bool {
        self.next == self.buffer.len()
    }
}