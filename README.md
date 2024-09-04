# Genp

Hey there, folks! It's time to unleash the power of Genp - the ultimate CLI tool for generating secure passwords and PINs that'll keep your data safer than a bank vault.

With this release, we've added some extra spice to make things even more fun. Who needs boring old passwords when you can have funky fresh ones straight from the Genp factory?

So, what are you waiting for? Grab your copy and start generating passwords that'll make your friends go "Whoa, how'd you come up with that?" 🎉

## Usage
```
A simple Generator for PINs and Passwords

Usage: genp [COMMAND]

Commands:
  PIN       Generates a PIN
  Password  Generates a Password
  help      Print this message or the help of the given subcommandv(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
### PIN
```
Generates a PIN

Usage: genp PIN [OPTIONS]

Options:
  -l, --length <length>  Sets the length of the PIN [default: 6]
  -s, --seed <seed>      Sets the seed for generating the PIN
  -h, --help             Print help
```

### Password

```
Generates a Password

Usage: genp Password [OPTIONS]

Options:
  -l, --length <length>  Sets the length of the password [default: 16]
  -s, --seed <seed>      Sets the seed for generating the password
  -h, --help             Print help
```
