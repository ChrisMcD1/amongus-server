#[derive(Debug, PartialEq, Clone, Default)]
pub struct ConfidentialData<T, U> {
    public_data: T,
    private_data: Option<T>,
    users_aware_of_private: Vec<U>,
}

impl<T, U> ConfidentialData<T, U>
where
    T: Copy,
    U: PartialEq + Copy,
{
    pub fn new(initial_data: T) -> Self {
        ConfidentialData {
            public_data: initial_data,
            private_data: None,
            users_aware_of_private: vec![],
        }
    }
    pub fn get(&self, user_id: U) -> T {
        match &self.private_data {
            Some(private) => {
                if self.users_aware_of_private.contains(&user_id) {
                    private.clone()
                } else {
                    self.public_data
                }
            }
            None => self.public_data,
        }
    }
    pub fn make_public(&mut self) {
        match &self.private_data {
            Some(private) => {
                self.public_data = *private;
            }
            None => {
                println!("We shouldn't be at this state. We are making confidential data public, but it already was public")
            }
        };
        self.wipe_private_data();
    }
    pub fn give_user_private_access(&mut self, user_id: &U) {
        self.users_aware_of_private.push(*user_id);
    }
    pub fn set_private_data(&mut self, private_data: T, users_aware_of_private: &[U]) {
        self.private_data = Some(private_data);
        self.users_aware_of_private = users_aware_of_private.to_vec();
    }
    pub fn set_public_data_and_reveal(&mut self, true_data: T) {
        self.public_data = true_data;
        self.wipe_private_data();
    }
    pub fn get_true_state(&self) -> T {
        if let Some(private) = self.private_data {
            return private;
        }
        return self.public_data;
    }
    fn wipe_private_data(&mut self) {
        self.private_data = None;
        self.users_aware_of_private = vec![];
    }
}
