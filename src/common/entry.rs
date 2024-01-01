pub struct Entry {
    pub term : u64,
    pub index : u64,
    pub prev_index: u64,
}

impl Entry {

    pub fn new(term: u64, index: u64, prev_index: u64) -> Self  {
        let entry = Entry {
            term,
            index,
            prev_index
        };
        entry
    }

}