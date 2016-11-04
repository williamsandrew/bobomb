// Testing macros

macro_rules! assert_status_set {
    ($c:expr, $f:expr) => {
        assert!($c.SR.is_set($f), "expected '{:?}' status register to be set", $f);
    }
}

macro_rules! assert_status_reset {
    ($c:expr, $f:expr) => {
        assert!(!$c.SR.is_set($f), "expected '{:?}' status register to be set", $f);
    }
}

macro_rules! assert_cpu_register {
    ($c:expr, $r:expr, $v:expr) => {
        let z = $c.register_value($r);
        assert!(z == $v, "expected '{:?}', got {:?} in register {:?}", $v, z, $r);
    }
}
