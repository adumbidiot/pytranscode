mod pystd;
use pystd::*;
fn main() {
    print(&[(&PyObject::from("Welcome to Aiden and Tad's Fabulous Kinematics Calculator!"))]);
    print(&[(&PyObject::from("Input the number of the equation you want"))]);
    print(&[(&PyObject::from(" 1: v=vo+at"))]);
    print(&[(&PyObject::from(" 2: v^2=vo^2+2a(xo+x)"))]);
    print(&[(&PyObject::from(" 3: w=wo+\u{3b1}t"))]);
    print(&[(&PyObject::from(" 4: w^2=wo^2+2a(\u{19f}-\u{19f}i)"))]);
    print(&[(&PyObject::from(" 5: V=IR"))]);
    print(&[(&PyObject::from(" 6: ac=v^2/r"))]);
    print(&[(&PyObject::from(" 7: Ek=1/2mv^2"))]);
    print(&[(&PyObject::from(" 8: Us=1/2kx^2"))]);
    print(&[(&PyObject::from(" 9: Ek=1/2iw^2"))]);
    print(&[(&PyObject::from("10: Ug=mgh"))]);
    let equation = input(&[(&PyObject::from(""))]);
    if equation == PyObject::from("1") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("v=vo+at"))]);
        let vel = input(&[(&PyObject::from("v:"))]);
        let vi = input(&[(&PyObject::from("vo:"))]);
        let acc = input(&[(&PyObject::from("a:"))]);
        let time = input(&[(&PyObject::from("t:"))]);
        if vel == PyObject::from("x") {
            print(&[
                (&PyObject::from("Final Velocity =")),
                (&float(&[(&vi)]).add(&float(&[(&acc)]).mul(&float(&[(&time)])))),
            ]);
        };
        if vi == PyObject::from("x") {
            let vt = float(&[(&acc)]).mul(&float(&[(&time)]));
            print(&[
                (&PyObject::from("Initial Velocity =")),
                (&float(&[(&vel)]).sub(&float(&[(&vt)]))),
            ]);
        };
        if acc == PyObject::from("x") {
            let vv = float(&[(&vel)]).sub(&float(&[(&vi)]));
            print(&[
                (&PyObject::from("Acceleration =")),
                (&float(&[(&vv)]).div(&float(&[(&time)]))),
            ]);
        };
        if time == PyObject::from("x") {
            print(&[
                (&PyObject::from("Time =")),
                (&float(&[(&vel)])
                    .sub(&float(&[(&vi)]))
                    .div(&float(&[(&acc)]))),
            ]);
        };
    };
    if equation == PyObject::from("2") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("v^2=vo^2+2a(xo+x)"))]);
        let vel = input(&[(&PyObject::from("v:"))]);
        let vi = input(&[(&PyObject::from("vo:"))]);
        let acc = input(&[(&PyObject::from("a:"))]);
        let xi = input(&[(&PyObject::from("xo:"))]);
        let x = input(&[(&PyObject::from("x:"))]);
        if vel == PyObject::from("x") {
            print(&[
                (&PyObject::from("Final Velocity =")),
                (&float(&[(&xi)])
                    .add(&float(&[(&x)]))
                    .mul(&float(&[(&acc)]))
                    .mul(&PyObject::from(2i64))
                    .add(&float(&[(&vi)]).pow(&PyObject::from(2i64)))
                    .pow(&PyObject::from(0.5f64))),
            ]);
        };
        if vi == PyObject::from("x") {
            print(&[
                (&PyObject::from("Initial Velocity=")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .sub(
                        &float(&[(&x)])
                            .sub(&float(&[(&xi)]))
                            .mul(&float(&[(&acc)]))
                            .mul(&PyObject::from(2i64)),
                    )
                    .pow(&PyObject::from(0.5f64))),
            ]);
        };
        if acc == PyObject::from("x") {
            print(&[
                (&PyObject::from("Acceleration =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .div(
                        &float(&[(&xi)])
                            .add(&float(&[(&x)]))
                            .mul(&PyObject::from(2i64)),
                    )
                    .add(&float(&[(&vi)]).pow(&PyObject::from(2i64)))),
            ]);
        };
        if xi == PyObject::from("x") {
            print(&[
                (&PyObject::from("Initial Position =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .sub(&float(&[(&vi)]).pow(&PyObject::from(2i64)))
                    .div(&float(&[(&acc)]).mul(&PyObject::from(2i64)))
                    .sub(&float(&[(&x)]))
                    .mul(&PyObject::from(1i64).sub(&PyObject::from(2i64)))),
            ]);
        };
        if x == PyObject::from("x") {
            print(&[
                (&PyObject::from("Final Position =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .sub(&float(&[(&vi)]).pow(&PyObject::from(2i64)))
                    .div(&float(&[(&acc)]).mul(&PyObject::from(2i64)))
                    .add(&float(&[(&xi)]))),
            ]);
        };
    };
    if equation == PyObject::from("5") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("V=IR"))]);
        let volt = input(&[(&PyObject::from("V:"))]);
        let cur = input(&[(&PyObject::from("I:"))]);
        let res = input(&[(&PyObject::from("R:"))]);
        if res == PyObject::from("x") {
            print(&[
                (&PyObject::from("Resistance =")),
                (&float(&[(&volt)]).div(&float(&[(&cur)]))),
            ]);
        };
        if cur == PyObject::from("x") {
            print(&[
                (&PyObject::from("Current =")),
                (&float(&[(&volt)]).div(&float(&[(&res)]))),
            ]);
        };
        if volt == PyObject::from("x") {
            print(&[
                (&PyObject::from("Voltage =")),
                (&float(&[(&cur)]).mul(&float(&[(&res)]))),
            ]);
        };
    };
    if equation == PyObject::from("3") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("w=wo+\u{3b1}t"))]);
        let rotvel = input(&[(&PyObject::from("w:"))]);
        let rotvi = input(&[(&PyObject::from("wo:"))]);
        let rotacc = input(&[(&PyObject::from("\u{3b1}:"))]);
        let time = input(&[(&PyObject::from("t:"))]);
        if rotvel == PyObject::from("x") {
            print(&[
                (&PyObject::from("Final Velocity =")),
                (&float(&[(&rotvi)]).add(&float(&[(&rotacc)]).mul(&float(&[(&time)])))),
            ]);
        };
        if rotvi == PyObject::from("x") {
            print(&[
                (&PyObject::from("Initial Velocity =")),
                (&float(&[(&rotvel)]).sub(&float(&[(&rotacc)]).mul(&float(&[(&time)])))),
            ]);
        };
        if rotacc == PyObject::from("x") {
            print(&[
                (&PyObject::from("Acceleration =")),
                (&float(&[(&rotvel)])
                    .sub(&float(&[(&rotvi)]))
                    .div(&float(&[(&time)]))),
            ]);
        };
        if time == PyObject::from("x") {
            print(&[
                (&PyObject::from("Time =")),
                (&float(&[(&rotvel)])
                    .sub(&float(&[(&rotvi)]))
                    .div(&float(&[(&rotacc)]))),
            ]);
        };
    };
    if equation == PyObject::from("6") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("ac=v^2/r"))]);
        let acc = input(&[(&PyObject::from("a:"))]);
        let vel = input(&[(&PyObject::from("v:"))]);
        let rad = input(&[(&PyObject::from("r:"))]);
        if acc == PyObject::from("x") {
            print(&[
                (&PyObject::from("Acceleration =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .div(&float(&[(&rad)]))),
            ]);
        };
        if vel == PyObject::from("x") {
            print(&[
                (&PyObject::from("Velocity =")),
                (&float(&[(&acc)]).mul(&float(&[(&rad)]))),
            ]);
        };
        if rad == PyObject::from("x") {
            print(&[
                (&PyObject::from("Radius =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .div(&float(&[(&acc)]))),
            ]);
        };
    };
    if equation == PyObject::from("4") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("w^2=wo^2+2a(\u{19f}-\u{19f}i)"))]);
        let vel = input(&[(&PyObject::from("w:"))]);
        let vi = input(&[(&PyObject::from("wo:"))]);
        let acc = input(&[(&PyObject::from("\u{3b1}:"))]);
        let xi = input(&[(&PyObject::from("\u{19f}i:"))]);
        let x = input(&[(&PyObject::from("\u{19f}:"))]);
        if vel == PyObject::from("x") {
            print(&[
                (&PyObject::from("Final Rotational Velocity =")),
                (&float(&[(&xi)])
                    .add(&float(&[(&x)]))
                    .mul(&float(&[(&acc)]))
                    .mul(&PyObject::from(2i64))
                    .add(&float(&[(&vi)]).pow(&PyObject::from(2i64)))
                    .pow(&PyObject::from(0.5f64))),
            ]);
        };
        if vi == PyObject::from("x") {
            print(&[
                (&PyObject::from("Initial Rotational Velocity =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .sub(
                        &float(&[(&xi)])
                            .add(&float(&[(&x)]))
                            .mul(&float(&[(&acc)]))
                            .mul(&PyObject::from(2i64)),
                    )
                    .pow(&PyObject::from(0.5f64))),
            ]);
        };
        if acc == PyObject::from("x") {
            print(&[
                (&PyObject::from("Rotational Acceleration =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .div(
                        &float(&[(&xi)])
                            .add(&float(&[(&x)]))
                            .mul(&PyObject::from(2i64)),
                    )
                    .add(&float(&[(&vi)]).pow(&PyObject::from(2i64)))),
            ]);
        };
        if xi == PyObject::from("x") {
            print(&[
                (&PyObject::from("Initial Position =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .sub(&float(&[(&vi)]).pow(&PyObject::from(2i64)))
                    .div(&float(&[(&acc)]).mul(&PyObject::from(2i64)))
                    .sub(&float(&[(&x)]))
                    .mul(&PyObject::from(1i64).sub(&PyObject::from(2i64)))),
            ]);
        };
        if x == PyObject::from("x") {
            print(&[
                (&PyObject::from("Final Position =")),
                (&float(&[(&vel)])
                    .pow(&PyObject::from(2i64))
                    .sub(&float(&[(&vi)]).pow(&PyObject::from(2i64)))
                    .div(&float(&[(&acc)]).mul(&PyObject::from(2i64)))
                    .add(&float(&[(&xi)]))),
            ]);
        };
    };
    if equation == PyObject::from("7") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("Ek=mv^2"))]);
        let vel = input(&[(&PyObject::from("v:"))]);
        let mas = input(&[(&PyObject::from("m:"))]);
        let ek = input(&[(&PyObject::from("Ek:"))]);
        if vel == PyObject::from("x") {
            print(&[
                (&PyObject::from("Velocity =")),
                (&float(&[(&ek)])
                    .mul(&PyObject::from(2i64))
                    .div(&float(&[(&mas)]))
                    .pow(&PyObject::from(0.5f64))),
            ]);
        };
        if mas == PyObject::from("x") {
            print(&[
                (&PyObject::from("Mass =")),
                (&float(&[(&ek)])
                    .mul(&PyObject::from(2i64))
                    .div(&float(&[(&vel)]).pow(&PyObject::from(2i64)))),
            ]);
        };
        if ek == PyObject::from("x") {
            print(&[
                (&PyObject::from("Kinetic Energy =")),
                (&float(&[(&mas)])
                    .mul(&PyObject::from(0.5f64))
                    .mul(&float(&[(&vel)]).pow(&PyObject::from(2i64)))),
            ]);
        };
    };
    if equation == PyObject::from("10") {
        print ( & [ ( & PyObject :: from ( "Input the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("Ug=mgh"))]);
        let Ug = input(&[(&PyObject::from("Ug:"))]);
        let mass = input(&[(&PyObject::from("m:"))]);
        let grav = input(&[(&PyObject::from("g:"))]);
        let height = input(&[(&PyObject::from("h:"))]);
        if Ug == PyObject::from("x") {
            print(&[
                (&PyObject::from("Gravitational Potential Energy =")),
                (&float(&[(&mass)])
                    .mul(&float(&[(&grav)]))
                    .mul(&float(&[(&height)]))),
            ]);
        };
        if mass == PyObject::from("x") {
            let gh = float(&[(&grav)]).mul(&float(&[(&height)]));
            print(&[(&PyObject::from("Mass =")), (&float(&[(&Ug)]).div(&gh))]);
        };
        if grav == PyObject::from("x") {
            let mh = float(&[(&mass)]).mul(&float(&[(&height)]));
            print(&[
                (&PyObject::from("Gravitational Acceleration =")),
                (&float(&[(&Ug)]).div(&float(&[(&mh)]))),
            ]);
        };
        if height == PyObject::from("x") {
            let mg = float(&[(&mass)]).mul(&float(&[(&grav)]));
            print(&[(&PyObject::from("Height =")), (&float(&[(&Ug)]).div(&mg))]);
        };
    };
    if equation == PyObject::from("8") {
        print(&[(&PyObject::from("Us=1/2kx^2"))]);
        let vel = input(&[(&PyObject::from("x:"))]);
        let mas = input(&[(&PyObject::from("k:"))]);
        let ek = input(&[(&PyObject::from("Us:"))]);
        if vel == PyObject::from("x") {
            let tek = int(&[(&ek)]).mul(&PyObject::from(2i64));
            let tekm = int(&[(&tek)]).div(&int(&[(&mas)]));
            print(&[
                (&PyObject::from("Position =")),
                (&int(&[(&tekm)]).pow(&PyObject::from(0.5f64))),
            ]);
        };
        if mas == PyObject::from("x") {
            let tek = int(&[(&ek)]).mul(&PyObject::from(2i64));
            let vels = int(&[(&vel)]).pow(&PyObject::from(2i64));
            print(&[
                (&PyObject::from("Spring Constant =")),
                (&int(&[(&tek)]).div(&int(&[(&vels)]))),
            ]);
        };
        if ek == PyObject::from("x") {
            let hm = int(&[(&mas)]).mul(&PyObject::from(0.5f64));
            let vels = int(&[(&vel)]).pow(&PyObject::from(2i64));
            print(&[
                (&PyObject::from("Elastic Potential Energy =")),
                (&int(&[(&hm)]).mul(&int(&[(&vels)]))),
            ]);
        };
    };
    if equation == PyObject::from("9") {
        print ( & [ ( & PyObject :: from ( "Imput the variables you know, and let me calculate the one you don't! Use x for unknown variable" ) ) ] ) ;
        print(&[(&PyObject::from("Ek=1/2iw^2"))]);
        let vel = input(&[(&PyObject::from("w:"))]);
        let mas = input(&[(&PyObject::from("i:"))]);
        let ek = input(&[(&PyObject::from("Ek:"))]);
        if vel == PyObject::from("x") {
            let tek = int(&[(&ek)]).mul(&PyObject::from(2i64));
            let tekm = int(&[(&tek)]).div(&int(&[(&mas)]));
            print(&[
                (&PyObject::from("Angular Velocity =")),
                (&int(&[(&tekm)]).pow(&PyObject::from(0.5f64))),
            ]);
        };
        if mas == PyObject::from("x") {
            let tek = int(&[(&ek)]).mul(&PyObject::from(2i64));
            let vels = int(&[(&vel)]).pow(&PyObject::from(2i64));
            print(&[
                (&PyObject::from("Rotational Inertia =")),
                (&int(&[(&tek)]).div(&int(&[(&vels)]))),
            ]);
        };
        if ek == PyObject::from("x") {
            let hm = int(&[(&mas)]).mul(&PyObject::from(0.5f64));
            let vels = int(&[(&vel)]).pow(&PyObject::from(2i64));
            print(&[
                (&PyObject::from("Rotational Kinetic Energy =")),
                (&int(&[(&hm)]).mul(&int(&[(&vels)]))),
            ]);
        };
    } else {
        if equation > PyObject::from("10") {
            print(&[(&PyObject::from("Sorry, invalid equation"))]);
        };
    };
}