# **ink! Cross Contract Calling**

This is an attempt to follow the tutorial from https://use.ink/basics/cross-contract-calling/
<br>
It contains other_contract which has a get_value
<br>
and MyContract which tries to instantiate other_contract and then store it so it can call the get_value

<br>
The other_contract  builds fine via $ cargo +nightly contract build
<br>
<br>
The MyContract  fails to build 
<br>

![plot](./Printscreens/Error_Cross_Contract.png)

<br>
cargo --version
<br>
cargo 1.65.0 (4bc8f24d3 2022-10-20)
<br>
<br>
cargo contract --version
<br>
cargo-contract 1.5.1-unknown-x86_64-apple-darwin
<br>
