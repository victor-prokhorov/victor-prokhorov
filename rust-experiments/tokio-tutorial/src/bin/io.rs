use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer_arr = [0; 10];
    let mut buffer = Vec::new();

    let mut file = File::create("bar.txt").await?;
    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);

    // read up to 10 bytes
    let r = f.read(&mut buffer_arr[..]).await?;
    // println!("The bytes: {:?}", &buffer[..r]);

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer);

    let mut reader: &[u8] = b"hello\n";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
