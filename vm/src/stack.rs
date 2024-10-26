use crate::opcode::{ Get, OpcodeExecutor };

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Stack(Vec<u128>);

impl AsRef<Vec<u128>> for Stack {
    fn as_ref(&self) -> &Vec<u128> {
        &self.0
    }
}

impl AsMut<Vec<u128>> for Stack {
    fn as_mut(&mut self) -> &mut Vec<u128> {
        &mut self.0
    }
}

impl From<Vec<u128>> for Stack {
    fn from(vec: Vec<u128>) -> Self {
        Self(vec)
    }
}

impl From<Stack> for Vec<u128> {
    fn from(stack: Stack) -> Self {
        stack.0
    }
}

impl std::ops::Deref for Stack {
    type Target = Vec<u128>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::iter::FromIterator<u128> for Stack {
    fn from_iter<I: IntoIterator<Item = u128>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl std::iter::IntoIterator for Stack {
    type Item = u128;
    type IntoIter = std::vec::IntoIter<u128>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl OpcodeExecutor for Stack {
    fn add(&mut self) {
        let (a, b) = self.pop2_sorted();

        let result = a.checked_add(b).expect("aritmathic overflow or underflow");

        self.push(result)
    }

    fn sub(&mut self) {
        let (a, b) = self.pop2_sorted();

        let result = a.checked_sub(b).expect("aritmathic overflow or underflow");

        self.push(result)
    }

    fn div(&mut self) {
        let (a, b) = self.pop2_sorted();

        let result = a.checked_div(b).expect("aritmathic overflow, underflow, or devision by zero");

        self.push(result);
    }

    fn mul(&mut self) {
        let (a, b) = self.pop2_sorted();

        let result = a.checked_mul(b).expect("aritmathic overflow or underflow");

        self.push(result);
    }

    fn modu(&mut self) {
        let (a, b) = self.pop2_sorted();

        self.0.push(a % b);
    }

    fn push(&mut self, value: u128) {
        self.0.push(value);
    }

    fn pop(&mut self) {
        self.0.pop();
    }
}

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn pop2_sorted(&mut self) -> (u128, u128) {
        let a = self.0.pop().unwrap_or_default();
        let b = self.0.pop().unwrap_or_default();

        (b, a)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u128> {
        self.0.iter()
    }
}

impl Get<Vec<u128>> for Stack {
    fn get(&self) -> Vec<u128> {
        self.0.clone()
    }
}
