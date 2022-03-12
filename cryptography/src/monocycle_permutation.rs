pub struct MonocyclePermutation {
    key: Vec<usize>
}

impl MonocyclePermutation {
    pub fn with_key(key: &Vec<usize>) -> MonocyclePermutation {
        MonocyclePermutation {
            key: key.clone()
        }
    }

    pub fn encrypt(&self, message: &Vec<char>) -> Vec<char> {
        let mut result = vec!['~'; message.len()];
        let mut key_index = 0;
        let last_block_index = result.len() / self.key.len() * self.key.len();
        for result_index in 0..last_block_index {
            let block_index = result_index / self.key.len();
            let message_index = block_index * self.key.len() + self.key[key_index] - 1;
            result[result_index] = message[message_index];
            key_index = (key_index + 1) % self.key.len();
        }
        for result_index in last_block_index..result.len() {
            loop {
                let message_index = last_block_index + self.key[key_index] - 1;
                if message_index >= message.len() {
                    key_index += 1;
                    continue;
                }
                result[result_index] = message[message_index];
                break;
            }
            key_index += 1;
        }
        result
    }

    pub fn decrypt(&self, message: &Vec<char>) -> Vec<char> {
        let mut result = vec!['~'; message.len()];
        let mut key_index = 0;
        let last_block_index = result.len() / self.key.len() * self.key.len();
        for message_index in 0..last_block_index {
            let block_index = message_index / self.key.len();
            let result_index = block_index * self.key.len() + self.key[key_index] - 1;
            result[result_index] = message[message_index];
            key_index = (key_index + 1) % self.key.len();
        }
        for message_index in last_block_index..message.len() {
            loop {
                let result_index = last_block_index + self.key[key_index] - 1;
                if result_index >= result.len() {
                    key_index += 1;
                    continue;
                }
                result[result_index] = message[message_index];
                break;
            }
            key_index += 1
        }
        result
    }
}
