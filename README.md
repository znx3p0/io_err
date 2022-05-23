# io_err

A serializable version of `std::io::Error` and `std::io::Result` with much better ergonomics.

```rust
fn test() -> Result<()> {
    let res: u32 = may_error()?;
    let another: Result<(), &'static str> = Err("error!");
    let map_another: Result<()> = another.map_err(err!(@other))?;
    err!((other, "error"))?; // == err!(("error"))?;
    Ok(())
}

fn return_error() -> Result<()> {
    err!(("unknown")) // == Err(err!("unknown"))
}

fn return_error_with_kind() -> Result<()> {
    err!((permission_denied, "unknown")) // == Err(err!(permission_denied, "unknown"))
}

fn simple_error() -> Result<()> {
    let err: Error = err!("unknown");
    Err(err)
}

fn simple_error_with_kind() -> Result<()> {
    let err: Error = err!(permission_denied, "unknown");
    Err(err)
}

fn errors() -> Result<()> {
    let err = "unknown";
    bail!("error {:?}", err); // == return err!(("error {:?}", err));
    Ok(())
}

fn permission_denied() -> Result<()> {
    let err = "denied >:(";
    bail!(permission_denied, "error {:?}", err);
    Ok(())
}
```
