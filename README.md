# Meaning

---

## 導入套件
```mn
::Meaning:IO, Math;
```

## 型別系統
```mn
Complex<integer, integer>; // simple type
// key is an integer type

Complex {
    real: integer;
    virtual: integer;
}; // interface system
// key can be a name or string
```

## 宣告變數
```mn
(Complex a) <- {0, 0};
```

## 宣告函數
```mn
add(Complex a, Complex b) -> Complex {
    return a + b; // or a + b
};
```

## 導出變數 (在前面加上 ')
```mn
(Complex 'a) <- {0, 0};

'add(Complex a, Complex b) -> Complex {
    a + b // or return a + b;
};
```

## 呼叫函數
```mn
'a <- (add a 'a);
```

## 條件判斷
```mn
if 'a = a {
    (IO::puts "data is same");
} else {
    (IO::puts "data is not same");
}
```

## 管道運算子
```mn
a |> IO::format |> IO::puts;
```

## for 迴圈
```mn
for i in 0..10 {
    i |> IO::format |> IO::puts;
}

// or

for IO::format |> IO::puts in 0..10;
```

## while 迴圈
```mn
while 'a != a {
    'a |> IO::format |> IO::puts;
    'a <- {0, 1};
}
```

## 屬性運算子
```mn
IO::format // :: is use to method

Math.PI // . is use to property
```

## 匿名函式
```mn
((Complex a, Complex b) -> Complex {
    a + b;
} 'a a);
```

## 定義macro
```mn
(macro hello() -> {
    () => {
        "hello" |> IO::puts;
    };
    (world) => {
        "hello world" |> IO::puts;
    };
})!; // println macro

(hello)!
(hello world)!
```
