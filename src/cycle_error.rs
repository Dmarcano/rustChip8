use std::fmt; 
use std::error; 

pub struct CycleError { 
    pub message : String 
}

impl fmt::Display for CycleError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ 
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for CycleError { 
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "CycleError{{message: {} }}", self.message) 
    }
}

impl error::Error for CycleError { 
    // the cycle error has no source since it is a generic error 
    fn source(&self) -> Option<&(dyn error::Error + 'static)> { 
        None
    }
}