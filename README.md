Small CLI that extract a password from KeyChain and put it on the stdout

## Usage for AI API Keys

- Install it. 
- In KeyChain (see below command to open keychain)
Create a keychain password
    - item name: `ai-keys` 
    - account name: `default-keys`
    - Password the text below:
```    
export OPENAI_API_KEY="sk."
export GERMINI_API_KEY="gk....."    
```

In the bash/zsh/sh file have the following alias
```sh
# default keys
alias akeys="source <(key-cli get ai-keys default-keys)"
```


## Mac KeyChain

```
open "/System/Library/CoreServices/Applications/Keychain Access.app"
```


<br />

[This Repo](https://github.com/jeremychone/rust-key-cli)