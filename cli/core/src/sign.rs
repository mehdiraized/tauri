use tauri_bundler::sign::{generate_key, save_keypair, read_key_from_file, sign_file};
use std::path::{Path, PathBuf};

pub struct Signer {
   private_key: Option<String>,
   password: Option<String>,
   binary: Option<PathBuf>,
}

impl Default for Signer {
   fn default() -> Self {
     Self {
      private_key: None,
      password: None,
      binary: None,
     }
   }
}

impl Signer {
   pub fn new() -> Self {
      Default::default()
    }

    pub fn private_key(mut self, private_key: &str) -> Self {
      self.private_key = Some(private_key.to_owned());
      self
    }

    pub fn password(mut self, password: &str) -> Self {
      self.password = Some(password.to_owned());
      self
    }

    pub fn empty_password(mut self) -> Self {
      self.password = Some("".to_owned());
      self
    }

    pub fn binary(mut self, binary: &str) -> Self {
      self.binary = Some(Path::new(binary).to_path_buf());
      self
    }

    pub fn private_key_path(mut self, private_key: &str) -> Self {
      self.private_key = Some(read_key_from_file(Path::new(private_key)).expect("Unable to extract private key"));
      self
    }

    pub fn run(self) -> crate::Result<()> {
      if self.private_key.is_none() {
         return Err(anyhow::anyhow!(
           "Key generation aborted: Unable to find the private key".to_string(),
         ));
       }

       if self.password.is_none() {
         return Err(anyhow::anyhow!(
            "Please use --no-password to set empty password or add --password <password> if your private key have a password.".to_string(),
          ));
       }

       let (manifest_dir, signature) = sign_file(
         self.private_key.unwrap(),
         self.password.unwrap(),
         self.binary.unwrap(),
         false,
       )?;
 
       println!(
         "\nYour file was signed successfully, You can find the signature here:\n{}\n\nPublic signature:\n{}\n\nMake sure to include this into the signature field of your update server.",
         manifest_dir.display(),
         signature
       );

       Ok(())
    }
}

pub struct KeyGenerator {
   password: Option<String>,
   output_path: Option<PathBuf>,
   force: bool
}

impl Default for KeyGenerator {
   fn default() -> Self {
     Self {
      password: None,
      output_path: None,
      force: false
     }
   }
}

impl KeyGenerator {
   pub fn new() -> Self {
      Default::default()
    }

    pub fn empty_password(mut self) -> Self {
      self.password = Some("".to_owned());
      self
    }

    pub fn force(mut self) -> Self {
      self.force = true;
      self
    }

    pub fn password(mut self, password: &str) -> Self {
      self.password = Some(password.to_owned());
      self
    }

    pub fn output_path(mut self, output_path: &str) -> Self {
      self.output_path = Some(Path::new(output_path).to_path_buf());
      self
    }
    
    pub fn generate_keys(self) -> crate::Result<()> {
      let keypair = generate_key(self.password).expect("Failed to generate key");

      if let Some(output_path) = self.output_path {

        let (secret_path, public_path) = save_keypair(
          self.force,
          output_path,
          &keypair.sk,
          &keypair.pk,
        ).expect("Unable to write keypair");

        println!(
        "\nYour keypair was generated successfully\nPrivate: {} (Keep it secret!)\nPublic: {}\n---------------------------",
        secret_path.display(),
        public_path.display()
        )
      } else {
        println!(
          "\nYour secret key was generated successfully - Keep it secret!\n{}\n\n",
          keypair.sk
        );
        println!(
          "Your public key was generated successfully:\n{}\n\nAdd the public key in your tauri.conf.json\n---------------------------\n",
          keypair.pk
        );
      }

      println!("\nEnvironment variabled used to sign:\n`TAURI_PRIVATE_KEY`  Path or String of your private key\n`TAURI_KEY_PASSWORD`  Your private key password (optional)\n\nATTENTION: If you lose your private key OR password, you'll not be able to sign your update package and updates will not works.\n---------------------------\n");
   
      Ok(())
   }
}