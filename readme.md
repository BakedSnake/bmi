## BMI Calculator

#### Install

```
$ git clone https://github.com/BakedSnake/bmi 
$ cd bmi
$ make
$ sudo make install
```

#### Usage

Calculate BMI:


```
$ bmi -b
```

Calculate calories:

```
$ bmi -c < height > < weight > < age > < gender > < activity >

$ bmi -c 178 70 30 male light-active

$ bmi -c 175 63 29 female active
```

Activity choices are: `sedentary`, `light-active`, `active`, `very-active`, `super-active`.

Help:

```
$ bmi -h
```
