mod bindings;
use std::ffi::CString;


pub fn s_read(compound_bloc: String) -> Result<String,String> {
    let cb = CString::new(compound_bloc).expect("C:B Fail");
    let name = cb.into_raw();

    let gateway = &mut 1 as &mut i32;
    let chaine = CString::new("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("null");
    let value = chaine.into_raw();
    let status = &mut 1 as &mut i32;
    let reterr = &mut 1 as &mut i32;

    let size = unsafe {bindings::sread(gateway, name, value, status, reterr)};

    let code_error = *reterr;
    let result = match code_error {
            0 => Ok({
                let value_str = unsafe {CString::from_raw(value)};
                let name_str = unsafe {CString::from_raw(name)};
                //Debug
                println!("Gateway {} Name {:?} Value {:?} Status {:?} Reterr {:?} Longueur {}", gateway, name_str, value_str, status, reterr, size - 1);
                value_str.into_string().unwrap()
                }),
            25 => Err(format!("{code_error} Invalid gateway")),
            28 => Err(format!("{code_error} Type mismatch")),
            30 => Err(format!("{code_error} Unconfigured gateway")),
            34 => Err(format!("{code_error} String not found")),
            _  => Err(format!("{code_error}")),
        };

       result
}

pub fn s_write(compound_bloc: String, data: String) -> Result<i32,String> {
    let cb = CString::new(compound_bloc).expect("C:B Fail");
    let name = cb.into_raw();

    let gateway = &mut 1 as &mut i32;
    let chaine = CString::new(data).expect("Empty");
    let value = chaine.into_raw();
    let reterr = &mut 1 as &mut i32;

    let r_result = unsafe {bindings::swrite(gateway, name, value, reterr)};

    let code_error = *reterr;
    let result = match code_error {
            0 => Ok({
                let value_str = unsafe {CString::from_raw(value)};
                let name_str = unsafe {CString::from_raw(name)};
                //debug
                println!("Gateway {} Name {:?} Value {:?} Reterr {}", gateway, name_str, value_str, reterr);
                r_result
            }),
            25 => Err(format!("{code_error} Invalid gateway")),
            30 => Err(format!("{code_error} Unconfigured gateway")),
            34 => Err(format!("{code_error} String not found")),
            _  => Err(format!("{code_error}")),
        };

    result
}

pub fn u_read(compound_bloc: String, mut cast: i32) -> Result<Iaxval,String> {

    let nument = &mut 1 as &mut i32;
    let value_array = &mut 0 as &mut i32;
    let valtype_array = &mut cast as &mut i32;
    let cb = CString::new(compound_bloc).expect("C:B Fail");
    let name_array = cb.into_raw();

    let gw_array = &mut 1 as &mut i32;
    let status_array = &mut 1 as &mut i32;
    let error_array = &mut 1 as &mut i32;
    let reterr = &mut 0 as &mut i32;

    unsafe {bindings::uread(gw_array, nument, name_array, valtype_array, value_array, status_array, error_array, reterr)};

    let code_error = *reterr;

    let result = match code_error {
        0 => Ok({
            let i32_value =  *value_array;
    
            //let s_status = format!("{:b}", status);
            let r_valtype_array = *status_array & 0x0F;
            let valeur: Iaxval = match r_valtype_array {
                //Integer
                2 => conv_type_2(i32_value),
                //float
                3 => conv_type_3(i32_value),
                //Boolean
                5 => conv_type_5(i32_value),
                // Long
                6 => conv_type_6(i32_value),
                _ => Iaxval {
                    iacval: String::new(),
                    iacbval: false,
                    iawval: 0,
                    iaival: 0,
                    ialval: 0,
                    iarval: 0.0,
                    iaucval: String::new(),
                },
            };

            valeur
        }),
        1 => Err(format!("{code_error} unable to access object")),
        25 => Err(format!("{code_error} Invalid gateway")),
        26 => Err(format!("{code_error} At least one of entries has an error")),
        27 => Err(format!("{code_error} invalid number of entries")),
        28 => Err(format!("{code_error} Value type mismatch; the status and value are still returned")),
        30 => Err(format!("{code_error} unconfigured gateway")),
        36 => Err(format!("{code_error} Invalid value type")),
        _  => Err(format!("{code_error}")),
    };

    result
}

pub fn u_write(compound_bloc: String, mut cast: i32, value: f32) -> Result<(),String>  {
    let nument = &mut 1 as &mut i32;
    let value_array = &mut 0 as &mut i32;
    let valtyp_array = &mut cast as &mut i32;
    let cb = CString::new(compound_bloc).expect("C:B Fail");
    let name_array = cb.into_raw();

    let gw_array = &mut 1 as &mut i32;
    let status = &mut 1 as &mut i32;
    let error_array = &mut 1 as &mut i32;
    let reterr = &mut 0 as &mut i32;

    if *valtyp_array == 1 && (value >= 0.0 && value <= 1.0) { 
        let b_val = value as i32;
        *value_array = b_val;
        unsafe {bindings::uwrite(gw_array, nument, name_array, valtyp_array, value_array, error_array, reterr)};
    }; 

    //Integer (16bits)
    if *valtyp_array == 2 {
        let i16_value = value as i16;
        if i16_value < i16::MAX && i16_value > i16::MIN {
             let i32_value = value as i32;
            *value_array = i32_value;
            unsafe {bindings::uwrite(gw_array, nument, name_array, valtyp_array, value_array, error_array, reterr)};
        }
    };

    //Float

    if *valtyp_array == 3 {
        let i32_value = conv_type_3_write(value);
        *value_array = i32_value;
        unsafe {bindings::uwrite(gw_array, nument, name_array, valtyp_array, value_array, error_array, reterr)};
    };


    //Long (32bits)

    if *valtyp_array == 6 {
        let i32_value = value as i32;
        *value_array = i32_value;
        unsafe {bindings::uwrite(gw_array, nument, name_array, valtyp_array, value_array, error_array, reterr)};
    };

    let _r_type = *status & 0x0F;

    let code_error = *reterr;

    let result = match code_error {
        0 => Ok(()),
        1 => Err(format!("{code_error} unable to access object")),
        25 => Err(format!("{code_error} Invalid gateway")),
        26 => Err(format!("{code_error} At least one of entries has an error")),
        27 => Err(format!("{code_error} invalid number of entries")),
        30 => Err(format!("{code_error} unconfigured gateway")),
        36 => Err(format!("{code_error} Invalid value type")),
        _  => Err(format!("{code_error}")),
    };

    result
}

//Float Write
fn conv_type_3_write(valeur: f32) -> i32 {
    println!("conv_type_3_write {}", valeur);
    //let negative = valeur.is_sign_negative();
    // Conversion en u32
    let u32_value = valeur.to_ne_bytes();
    //conversion en i32
    let i32_value = i32::from_ne_bytes(u32_value);
    println!("i32_value {} {:b}", i32_value, i32_value);
    //if negative { i32_value = i32_value * -1 };
    println!("i32_value {} {:b}", i32_value, i32_value);
    i32_value
}

//Float
fn conv_type_3(valeur: i32) -> Iaxval {
    println!("conv_type_3 {}", valeur);
    let negative = valeur.is_negative();
    let mut resultat = 0.0_f32;
    if valeur != i32::MIN {
        if negative {
            // détermine son image positive
            let u_valeur: i32 = (i32::MIN - valeur).abs();
            resultat = f32::from_bits(u_valeur.try_into().unwrap()) * -1.0;
        } else {
        resultat = f32::from_bits(valeur.try_into().unwrap()); 
        }
    };

    Iaxval {
        iacval: String::new(),
        iacbval: false,
        iawval: 0,
        iaival: 0,
        ialval: resultat as i32,
        iarval: resultat,
        iaucval: String::new(),
    }
}

// Integer
fn conv_type_2(valeur: i32) -> Iaxval {
    println!("conv_type_2 {}", valeur);
    Iaxval {
        iacval: String::new(),
        iacbval: false,
        iawval: 0,
        iaival: valeur as i16,
        ialval: valeur,
        iarval: valeur as f32,
        iaucval: String::new(),
    }
}

// Long
fn conv_type_6(valeur: i32) -> Iaxval {
    println!("conv_type_6 {}", valeur);
    Iaxval {
        iacval: String::new(),
        iacbval: false,
        iawval: 0,
        iaival: 0,
        ialval: valeur,
        iarval: valeur as f32,
        iaucval: String::new(),
    }
}

// Boolean
fn conv_type_5(valeur: i32) -> Iaxval {
    println!("conv_type_5 {}", valeur);
    let mut type_bool = false;
    if valeur == 1 { type_bool = true };
    Iaxval {
        iacval: String::new(),
        iacbval: type_bool,
        iawval: type_bool as i8,
        iaival: type_bool as i16,
        ialval: valeur,
        iarval: valeur as f32,
        iaucval: String::new(),
    }
}

#[derive(Debug)]
pub struct Iaxval {
    iacval: String,
    iacbval: bool,
    iawval: i8,
    iaival: i16,
    ialval: i32,
    iarval: f32,
    iaucval: String,
} 

