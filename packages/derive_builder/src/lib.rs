use proc_macro::TokenStream;
use quote::quote;
use syn::{ DeriveInput, parse_macro_input };

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);

    // pub struct Command {
    //     executable: String,
    //     args: Vec<String>,
    //     env: Vec<String>,
    //     current_dir: String,
    // }

    let expanded =
        quote! {
    use color_eyre::{ eyre::eyre, Result };

    pub struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }

    impl CommandBuilder {
        fn executable(&mut self, executable: String) -> &mut Self {
            self.executable = Some(executable);
            self
        }

        fn args(&mut self, args: Vec<String>) -> &mut Self {
            self.args = Some(args);
            self
        }

        fn env(&mut self, env: Vec<String>) -> &mut Self {
            self.env = Some(env);
            self
        }

        fn current_dir(&mut self, current_dir: String) -> &mut Self {
            self.current_dir = Some(current_dir);
            self
        }

        pub fn build(&mut self) -> Result<Command> {
            let executable = match &self.executable {
                Some(executable) => { executable.clone() }
                None => {
                    return Err(eyre!(".executable() setter was never called on your builder"));
                }
            };

            let args = match &self.args {
                Some(args) => { args.clone() }
                None => {
                    return Err(eyre!(".args() setter was never called on your builder"));
                }
            };

            let env = match &self.env {
                Some(env) => { env.clone() }
                None => {
                    return Err(eyre!(".env() setter was never called on your builder"));
                }
            };

            let current_dir = match &self.current_dir {
                Some(current_dir) => { current_dir.clone() }
                None => {
                    return Err(eyre!(".current_dir() setter was never called on your builder"));
                }
            };

            Ok(Command { executable, args, env, current_dir })
        }
    }

    impl Command {
        pub fn builder() -> CommandBuilder {
            CommandBuilder {
                executable: None,
                args: None,
                env: None,
                current_dir: None,
            }
        }
    }
        };

    TokenStream::from(expanded)
}