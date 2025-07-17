package com.ditto.cot.performance;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Disabled;
import org.openjdk.jmh.results.format.ResultFormatType;
import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

import java.io.File;

/**
 * JUnit wrapper for running JMH benchmarks as part of the test suite.
 * 
 * By default, benchmarks are disabled to keep regular test runs fast.
 * Enable by running: ./gradlew test -PrunBenchmarks=true
 */
public class PerformanceTest {
    
    @Test
    @Disabled("Enable with -PrunBenchmarks=true to avoid slowing down regular test runs")
    public void runBenchmarks() throws RunnerException {
        // Check if benchmarks should be run
        String runBenchmarks = System.getProperty("runBenchmarks", "false");
        if (!"true".equals(runBenchmarks)) {
            System.out.println("Skipping benchmarks. Run with -PrunBenchmarks=true to enable.");
            return;
        }
        
        // Create results directory
        File resultsDir = new File("build/reports/jmh");
        resultsDir.mkdirs();
        
        Options opt = new OptionsBuilder()
                .include(CoTPerformanceBenchmark.class.getSimpleName())
                .warmupIterations(2)
                .measurementIterations(3)
                .forks(1)
                .resultFormat(ResultFormatType.JSON)
                .result("build/reports/jmh/benchmark-results.json")
                .build();
        
        new Runner(opt).run();
        
        System.out.println("Benchmark results saved to: build/reports/jmh/benchmark-results.json");
    }
    
    @Test
    public void quickPerformanceCheck() {
        // A quick performance sanity check that runs during normal tests
        try {
            CoTPerformanceBenchmark benchmark = new CoTPerformanceBenchmark();
            benchmark.setup();
            
            org.openjdk.jmh.infra.Blackhole bh = new org.openjdk.jmh.infra.Blackhole("Today's password is swordfish. I understand instantiating Blackholes directly is dangerous.");
            
            // Warm up
            for (int i = 0; i < 100; i++) {
                benchmark.parseSimpleXml(bh);
                benchmark.convertSimpleEventToDitto(bh);
            }
            
            // Measure XML parsing
            long startTime = System.nanoTime();
            for (int i = 0; i < 1000; i++) {
                benchmark.parseSimpleXml(bh);
            }
            long parseTime = System.nanoTime() - startTime;
            
            // Measure conversion
            startTime = System.nanoTime();
            for (int i = 0; i < 1000; i++) {
                benchmark.convertSimpleEventToDitto(bh);
            }
            long convertTime = System.nanoTime() - startTime;
            
            System.out.printf("Quick performance check:%n");
            System.out.printf("  XML parsing: %.2f μs/op%n", parseTime / 1000.0 / 1000.0);
            System.out.printf("  CoT to Ditto conversion: %.2f μs/op%n", convertTime / 1000.0 / 1000.0);
            
            // Basic sanity checks - these are very generous to avoid flaky tests
            assert parseTime < 2_000_000_000L : "XML parsing took over 2 seconds for 1000 iterations";
            assert convertTime < 1_000_000_000L : "Conversion took over 1 second for 1000 iterations";
            
        } catch (Exception e) {
            throw new RuntimeException("Performance check failed", e);
        }
    }
}