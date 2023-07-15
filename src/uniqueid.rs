static mut UNIQUEID_POOL: i32 = 0;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct UniqueID
{
	id: i32,
	// operator bool()const { return static_cast<bool>(id_); }
}

impl UniqueID {
    // Returns the internal unique identifier for this object.
	// The return value from this isn't really useful for
	//	much other than debug output. Nonetheless, that is
	//	one step above useless, so here it is. 
	pub fn get_uid(self) -> i32 { 
        self.id
    }


    pub fn new(id: Option<i32>) -> Self {
        match id {
            // explicit UniqueID(int id_):id_(id_) { }
            Some(id_) => Self{id: id_},

            // UniqueID():id_(next_id()) { }
            None => {
                let s = Self{id:0};
                Self{id: s.next_id()}
            }
        }
    }

	pub fn make_unique(&mut self) { 
        self.id = self.next_id()
    }

	// static const UniqueID nil() { return UniqueID(0); }
    pub fn nil(self) -> Self {
        Self{ id: 0}
    }

    // void mimic(const UniqueID& x) { id_=x.id_; }
    pub fn mimic(&mut self, x: Self) { 
        self.id=x.id; 
    }

    fn next_id(&self) -> i32 {
        unsafe {
            UNIQUEID_POOL += 1;
            UNIQUEID_POOL
        }
    }
}


// impl PartialEq for UniqueID {
//     // bool operator==(const UniqueID &rhs)const { return id_==rhs.id_; }
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//     }

// 	// bool operator!=(const UniqueID &rhs)const { return id_!=rhs.id_; }
//     fn ne(&self, other: &Self) -> bool {
//         self.id != other.id
//     }

//     // bool operator<(const UniqueID &rhs)const { return id_<rhs.id_; }
// }

// impl PartialOrd for UniqueID {
//     fn lt(&self, other: &Self) -> bool {
//         self.id < other.id
//     }
// } 