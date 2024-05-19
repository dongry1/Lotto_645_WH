# Lotto_645_WH

##### Description
1) get the user input for no of games.
2) user input validated, whether it is number or not
3) hashset is good for saving unique element.
4) in Cargo.toml, Added 
``` 
[build]
rustflags = ["-Z", "threads=4"]
```

```c
% cargo run
    # 645 Lotto.
    # Large number would take a while and use your computing power a lot.
    # This Program will generate 6 number between 1 and 45, Lotto Number.
    # How many games do you want to generate?  
2
Your input is verified : 2
1:{13, 16, 42, 14, 18, 41}
2:{38, 24, 9, 22, 34, 6}

% cargo run             
warning: unused manifest key: build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/eLotto_645_WH`

    # 645 Lotto.
    # Large number would take a while and use your computing power a lot.
    # This Program will generate 6 number between 1 and 45, Lotto Number.
    # How many games do you want to generate?  
text
Error, Only Positive Number is allowed. invalid digit found in string

% cargo run
warning: unused manifest key: build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/eLotto_645_WH`

    # 645 Lotto.
    # Large number would take a while and use your computing power a lot.
    # This Program will generate 6 number between 1 and 45, Lotto Number.
    # How many games do you want to generate?  
-=-=
Error, Only Positive Number is allowed. invalid digit found in string
```