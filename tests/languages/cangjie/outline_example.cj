package com.example.outline

import std.io.*
import std.collection.*

// A data structure
struct DataPoint {
    var label: String
    var value: Float64
}

// An interface for processors
interface DataProcessor {
    func process(data: DataPoint): DataPoint
}

// An enum of status codes
enum Status {
    | Ok
    | Warning(String)
    | Error(Int64)
}

class DataPipeline {
    var points: ArrayList<DataPoint>

    public init() {
        this.points = ArrayList<DataPoint>()
    }

    public func addPoint(p: DataPoint): Unit {
        points.add(p)
    }

    public prop length: Int64 {
        get() {
            return points.size
        }
    }

    public func processAll(processor: DataProcessor): Unit {
        for (i in 0..points.size) {
            if (let Some(p) <- points.get(i)) {
                let processed = processor.process(p)
                points[i] = processed
            }
        }
    }

    static init() {
        println("DataPipeline initialized")
    }

    ~init() {
        println("DataPipeline destroyed")
    }
}

operator func +(a: DataPoint, b: DataPoint): DataPoint {
    return DataPoint("${a.label}+${b.label}", a.value + b.value)
}

type PointMap = HashMap<String, DataPoint>

macro debugLog(msg: String) {
    println("[DEBUG] ${msg}")
}

extend DataPoint {
    public func describe(): String {
        return "${label}: ${value}"
    }
}
