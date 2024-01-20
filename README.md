# Rust physics Engine

## Enum of si units
### Prevents misspelling and increases code readability
#### You can print them

```
    println!("An apple is aproximately 1 {}",SiUnit::Kilogram);
    println!("A minute is aproximately 60 {}s",SiUnit::Second);
```

## Not enough
### Create a struct that holds an f64 and two vectors of si units one for the numerator and the other for the denominator

```
    let fast = Value{magnitude: 10_f64,si_units_num: Vec::from([SiUnit::Metre]),si_units_den: Vec::from([SiUnit::Second,SiUnit::Second])};
    let slow = Value{magnitude: 2_f64,si_units_num: Vec::from([SiUnit::Metre]),si_units_den: Vec::from([SiUnit::Second,SiUnit::Second])};
```

### Add Values
```
println!("{}",fast.clone()+slow.clone());
```

### Multiply values
#### Note the units change when preforming multiplication
```
println!("{}",fast.clone() * slow.clone());
```

### We can also divide
```
    let distance = Value{magnitude: 20_f64,si_units_num: Vec::from([SiUnit::Metre]),si_units_den: Vec::<SiUnit>::new()};
    let time = Value{magnitude: 2_f64,si_units_num: Vec::from([SiUnit::Second]),si_units_den: Vec::<SiUnit>::new()};

    let speed = distance/time;

    println!("Speed is {}",speed);
```
We get a Value representing speed without explicitly creating it.

