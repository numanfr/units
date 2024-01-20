# Rust physics Engine

## Enum of si units
### Prevents misspelling and increases code readability

1. Metre
2. Second
3. Kilogram
4. Ampere
5. Kelvin
6. Mole
7. Candela

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

## Instead of declaring the whole Value each time, we can use Value templates from the builtin enums DerivedUnits and DerivedQuantities

DerivedUnits

1. Hertz
2. Newtons
3. Pascals
4. Joules
5. Watts
6. Volts
7. Coulombs
8. Sieverts

Derived Quantities

1. Speed
2. Velocity
3. Acceleration
4. Area
5. Volume
6. Mass
7. Force
8. Time
9. Scalar
10. Distance

### The get_value function returns a Value type, and the set_magnitude function changes the magnitude.
```
let force = DerivedQuantities::Force.get_value().set_magnitude(15_f64);
let pressure = DerivedUnit::Pascals.get_value().set_magnitude(5_f64);

let area = force/pressure;

println!("{}",area);
```

### We can also check if the Value we get is indeed an area by comapring it with the builtin Area template using the same() function

assert!(area.same(&DerivedQuantities::Area.get_value()));

## You can also use some of the built in physical constants
```
let g =  SiConstant::GravitationalConstant.get_value();
let c = SiConstant::SpeedOfLight.get_value();

println!("Gravitational Constant is {}",g);
println!("Soeed of light is {}",c);
```

### We can derive earth's acceleration due to gravity using earth's mass, radius, and the gravitational constant.
#### g = Gm/(r^2) where g is the acceleration, G is the gravitational constant, m is the mass, r is the radius.
```
let earth_mass = DerivedQuantities::Mass.get_value().set_magnitude(5.972e24);
let earth_radius = DerivedQuantities::Distance.get_value().set_magnitude(6371e3);
let g = SiConstant::GravitationalConstant.get_value();

let acc = g*earth_mass/earth_radius.powi(2);

assert!(acc.same(&DerivedQuantities::Acceleration.get_value()));

println!("{}",acc);
```

## We can also represent physical vectors that contain direction
```
let v = Vector{value: DerivedQuantities::Force.get_value(),theta: PI};
println!("{}",v);
```
### We can add Vectors 
```
let car1 = Vector{value: DerivedQuantities::Force.get_value(),theta: 0_f64};
let car2 = Vector{value: DerivedQuantities::Force.get_value(),theta: PI/2.0};
let collision = car1+car2;
println!("{}",collision);
```
### We can multiply
```
let v1 = Vector{value: DerivedQuantities::Force.get_value(),theta: 0_f64};
let v2 = Vector{value: DerivedQuantities::Force.get_value(),theta: PI/2.0};
let product = v1*v2;
println!("{}",product);
```
