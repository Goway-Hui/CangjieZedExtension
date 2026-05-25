package com.example.main

import std.io.*

class MainClass {
    var name: String
    static var count: Int64 = 0

    public init(name: String) {
        this.name = name
        count += 1
    }

    public func greet(): String {
        return "Hello, ${name}!"
    }

    public func getName(): String {
        return name
    }

    static func getCount(): Int64 {
        return count
    }
}

main() {
    let instance = MainClass("World")
    println(instance.greet())
    println("Total instances: ${MainClass.getCount()}")
}

func helper(): Bool {
    return true
}

struct Point {
    var x: Float64
    var y: Float64

    public init(x: Float64, y: Float64) {
        this.x = x
        this.y = y
    }

    public func distance(): Float64 {
        return (x ** 2.0 + y ** 2.0) ** 0.5
    }

    mut func translate(dx: Float64, dy: Float64): Unit {
        x += dx
        y += dy
    }
}

interface Drawable {
    func draw(): Unit
}

enum Color {
    | Red
    | Green
    | Blue
    | Custom(Float64, Float64, Float64)
}

extend Point <: Drawable {
    public func draw(): Unit {
        println("Point at (${x}, ${y})")
    }
}

class Circle <: Point {
    var radius: Float64

    public init(x: Float64, y: Float64, radius: Float64) {
        super.init(x, y)
        this.radius = radius
    }

    public override func distance(): Float64 {
        return radius
    }
}
