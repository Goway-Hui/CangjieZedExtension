package com.example.test

import std.io.*
import std.test.*

class Calculator {
    public func add(a: Int64, b: Int64): Int64 {
        return a + b
    }

    public func subtract(a: Int64, b: Int64): Int64 {
        return a - b
    }

    public func multiply(a: Int64, b: Int64): Int64 {
        return a * b
    }

    public func divide(a: Int64, b: Int64): Int64 {
        return a / b
    }
}

class CalculatorTest {
    var calc: Calculator

    public init() {
        this.calc = Calculator()
    }

    @Test
    public func testAdd(): Unit {
        let result = calc.add(2, 3)
        assert(result == 5)
    }

    @Test
    public func testSubtract(): Unit {
        let result = calc.subtract(10, 3)
        assert(result == 7)
    }

    @Test
    public func testMultiply(): Unit {
        let result = calc.multiply(4, 5)
        assert(result == 20)
    }

    @Test
    public func testDivide(): Unit {
        let result = calc.divide(10, 2)
        assert(result == 5)
    }
}

// A generic utility class
class StringUtils {
    static func reverse(s: String): String {
        var result = ""
        for (rune in s.runes()) {
            result = "${rune}${result}"
        }
        return result
    }

    static func isBlank(s: String): Bool {
        return s.trimAscii().size == 0
    }
}
