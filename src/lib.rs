use std::sync::atomic::AtomicPtr;

pub struct HazPtrHolder;
pub struct HazPtr;
pub struct HazPtrObject;

// Holds linked list of HazPtr
pub struct HazPrtDomain;

pub struct SharedHazPtrDomain;

struct Retired;
