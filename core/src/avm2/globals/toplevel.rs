//! Global scope built-ins

use ruffle_wstr::Units;

use crate::avm2::activation::Activation;
use crate::avm2::object::Object;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::string::{AvmString, WStr, WString};
use crate::stub::Stub;
use ruffle_wstr::Integer;
use std::borrow::Cow;
use std::fmt::Write;

pub fn trace<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    match args {
        [] => activation.context.avm_trace(""),
        [arg] => {
            let msg = arg.coerce_to_string(activation)?;
            activation.context.avm_trace(&msg.to_utf8_lossy());
        }
        args => {
            let strings = args
                .iter()
                .map(|a| a.coerce_to_string(activation))
                .collect::<Result<Vec<_>, _>>()?;
            let msg = crate::string::join(&strings, &WStr::from_units(b" "));
            activation.context.avm_trace(&msg.to_utf8_lossy());
        }
    }

    Ok(Value::Undefined)
}

pub fn log_warn<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    match args {
        [] => tracing::warn!("(__ruffle__.log_warn called with no arg)"),
        [arg] => {
            let msg = arg.coerce_to_string(activation)?;
            tracing::warn!("{}", &msg.to_utf8_lossy());
        }
        args => {
            let strings = args
                .iter()
                .map(|a| a.coerce_to_string(activation))
                .collect::<Result<Vec<_>, _>>()?;
            let msg = crate::string::join(&strings, &WStr::from_units(b" "));
            tracing::warn!("{}", &msg.to_utf8_lossy());
        }
    }

    Ok(Value::Undefined)
}

pub fn stub_method<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    match args {
        [class, method] => {
            let class = class.coerce_to_string(activation)?;
            let method = method.coerce_to_string(activation)?;
            activation
                .context
                .stub_tracker
                .encounter(&Stub::Avm2Method {
                    class: Cow::Owned(class.to_utf8_lossy().to_string()),
                    method: Cow::Owned(method.to_utf8_lossy().to_string()),
                    specifics: None,
                });
        }
        [class, method, specifics] => {
            let class = class.coerce_to_string(activation)?;
            let method = method.coerce_to_string(activation)?;
            let specifics = specifics.coerce_to_string(activation)?;
            activation
                .context
                .stub_tracker
                .encounter(&Stub::Avm2Method {
                    class: Cow::Owned(class.to_utf8_lossy().to_string()),
                    method: Cow::Owned(method.to_utf8_lossy().to_string()),
                    specifics: Some(Cow::Owned(specifics.to_utf8_lossy().to_string())),
                });
        }
        _ => tracing::warn!("(__ruffle__.stub_method called with wrong args)"),
    }

    Ok(Value::Undefined)
}

pub fn stub_getter<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    match args {
        [class, property] => {
            let class = class.coerce_to_string(activation)?;
            let property = property.coerce_to_string(activation)?;
            activation
                .context
                .stub_tracker
                .encounter(&Stub::Avm2Getter {
                    class: Cow::Owned(class.to_utf8_lossy().to_string()),
                    property: Cow::Owned(property.to_utf8_lossy().to_string()),
                });
        }
        _ => tracing::warn!("(__ruffle__.stub_getter called with wrong args)"),
    }

    Ok(Value::Undefined)
}

pub fn stub_setter<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    match args {
        [class, property] => {
            let class = class.coerce_to_string(activation)?;
            let property = property.coerce_to_string(activation)?;
            activation
                .context
                .stub_tracker
                .encounter(&Stub::Avm2Setter {
                    class: Cow::Owned(class.to_utf8_lossy().to_string()),
                    property: Cow::Owned(property.to_utf8_lossy().to_string()),
                });
        }
        _ => tracing::warn!("(__ruffle__.stub_setter called with wrong args)"),
    }

    Ok(Value::Undefined)
}

pub fn stub_constructor<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    match args {
        [class] => {
            let class = class.coerce_to_string(activation)?;
            activation
                .context
                .stub_tracker
                .encounter(&Stub::Avm2Constructor {
                    class: Cow::Owned(class.to_utf8_lossy().to_string()),
                    specifics: None,
                });
        }
        [class, specifics] => {
            let class = class.coerce_to_string(activation)?;
            let specifics = specifics.coerce_to_string(activation)?;
            activation
                .context
                .stub_tracker
                .encounter(&Stub::Avm2Constructor {
                    class: Cow::Owned(class.to_utf8_lossy().to_string()),
                    specifics: Some(Cow::Owned(specifics.to_utf8_lossy().to_string())),
                });
        }
        _ => tracing::warn!("(__ruffle__.stub_constructor called with wrong args)"),
    }

    Ok(Value::Undefined)
}

pub fn is_finite<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(val) = args.get(0) {
        Ok(val.coerce_to_number(activation)?.is_finite().into())
    } else {
        Ok(false.into())
    }
}

pub fn is_nan<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(val) = args.get(0) {
        Ok(val.coerce_to_number(activation)?.is_nan().into())
    } else {
        Ok(true.into())
    }
}

pub fn parse_int<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let string = match args.get(0).unwrap_or(&Value::Undefined) {
        Value::Undefined => "null".into(),
        value => value.coerce_to_string(activation)?,
    };

    let radix = match args.get(1) {
        Some(value) => value.coerce_to_i32(activation)?,
        None => 0,
    };

    let result = crate::avm2::value::string_to_int(&string, radix, false);
    Ok(result.into())
}

pub fn parse_float<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(value) = args.get(0) {
        let string = value.coerce_to_string(activation)?;
        let swf_version = activation.context.swf.version();
        if let Some(result) = crate::avm2::value::string_to_f64(&string, swf_version, false) {
            return Ok(result.into());
        }
    }

    Ok(f64::NAN.into())
}

pub fn escape<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let value = match args.first() {
        None => return Ok("undefined".into()),
        Some(Value::Undefined) => return Ok("null".into()),
        Some(value) => value,
    };

    let mut output = WString::new();

    // Characters that are not escaped, sourced from as3 docs
    let not_converted =
        WStr::from_units(b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@-_.*+/");

    for x in value.coerce_to_string(activation)?.iter() {
        if not_converted.contains(x) {
            output.push(x);
        } else {
            let encode = if x <= u8::MAX.into() {
                format!("%{x:02X}")
            } else {
                format!("%u{x:04X}")
            };
            output.push_str(WStr::from_units(encode.as_bytes()));
        }
    }

    Ok(AvmString::new(activation.context.gc_context, output).into())
}

pub fn unescape<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let value = match args.first() {
        None => return Ok("undefined".into()),
        Some(Value::Undefined) => return Ok("null".into()),
        Some(value) => value.coerce_to_string(activation)?,
    };

    let mut output = WString::new();
    let mut index = 0;
    while let Some(byte) = value.get(index) {
        index += 1;
        if byte != b'%' as u16 {
            output.push(byte);
            continue;
        }

        let prev_index = index;
        let len = match value.get(index) {
            // 0x75 == 'u'
            Some(0x75) => {
                // increment one to consume the 'u'
                index += 1;
                4
            }
            _ => 2,
        };

        if let Some(x) = value
            .slice(index..)
            .and_then(|v| v.slice(..len))
            .and_then(|v| u32::from_wstr_radix(v, 16).ok())
        {
            // NOTE: Yes, unpaired surrogates are allowed
            output.push(x as u16);
            index += len;
        } else {
            output.push(b'%' as u16);
            index = prev_index;
        }
    }
    Ok(AvmString::new(activation.context.gc_context, output).into())
}

pub fn encode_uri<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    encode_utf8_with_exclusions(
        activation,
        args,
        // Characters that are not escaped, sourced from as3 docs
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@;/?:@&=+$,#-_.!~*'()",
    )
}

pub fn encode_uri_component<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    encode_utf8_with_exclusions(
        activation,
        args,
        // Characters that are not escaped, sourced from as3 docs
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_.!~*'()",
    )
}

fn encode_utf8_with_exclusions<'gc>(
    activation: &mut Activation<'_, 'gc>,
    args: &[Value<'gc>],
    not_converted: &str,
) -> Result<Value<'gc>, Error<'gc>> {
    let value = match args.first() {
        None => return Ok("undefined".into()),
        Some(Value::Undefined) => return Ok("null".into()),
        Some(value) => value,
    };

    let mut output = String::new();

    let input = value.coerce_to_string(activation)?;
    let input_string = match input.units() {
        // Latin-1 values map directly to unicode codepoints,
        // so we can directly convert to a `char`
        Units::Bytes(bytes) => bytes.iter().map(|b| *b as char).collect(),
        Units::Wide(wide) => String::from_utf16_lossy(wide),
    };

    for x in input_string.chars() {
        if not_converted.contains(x) {
            output.push(x);
        } else {
            let mut bytes = [0; 4];
            let utf8_bytes = x.encode_utf8(&mut bytes);
            let mut encoded = String::new();
            // Each byte in the utf-8 encoding is encoded as a hex value
            for byte in utf8_bytes.bytes() {
                write!(encoded, "%{x:02X}", x = byte).unwrap();
            }
            output.push_str(&encoded);
        }
    }

    Ok(AvmString::new_utf8(activation.context.gc_context, output).into())
}
