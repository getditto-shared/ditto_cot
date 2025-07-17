package com.ditto.cot.performance;

import com.ditto.cot.CoTConverter;
import com.ditto.cot.CoTEvent;
import com.fasterxml.jackson.databind.ObjectMapper;
import jakarta.xml.bind.JAXBException;
import org.openjdk.jmh.annotations.*;
import org.openjdk.jmh.infra.Blackhole;
import org.openjdk.jmh.runner.Runner;
import org.openjdk.jmh.runner.RunnerException;
import org.openjdk.jmh.runner.options.Options;
import org.openjdk.jmh.runner.options.OptionsBuilder;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeUnit;

/**
 * Performance benchmarks for CoT to Ditto conversions.
 * 
 * Run with: ./gradlew test --tests CoTPerformanceBenchmark
 */
@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.MICROSECONDS)
@State(Scope.Benchmark)
@Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
@Warmup(iterations = 3, time = 1)
@Measurement(iterations = 5, time = 1)
public class CoTPerformanceBenchmark {
    
    private static final String SIMPLE_COT_XML = """
        <?xml version='1.0' encoding='UTF-8'?>
        <event version='2.0' uid='ANDROID-TEST-123' type='a-f-G-U-C' 
               time='2024-01-15T10:30:00.000Z' start='2024-01-15T10:30:00.000Z' 
               stale='2024-01-15T11:00:00.000Z' how='h-g-i-g-o'>
            <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
            <detail>
                <contact callsign='ALPHA-1' endpoint='192.168.1.100:4242:tcp'/>
                <__group name='Blue Team' role='Squad Leader'/>
            </detail>
        </event>
        """;
    
    private static final String COMPLEX_COT_XML = """
        <?xml version='1.0' encoding='UTF-8'?>
        <event version='2.0' uid='COMPLEX-TEST-456' type='a-f-G-U-C' 
               time='2024-01-15T10:30:00.000Z' start='2024-01-15T10:30:00.000Z' 
               stale='2024-01-15T11:00:00.000Z' how='h-g-i-g-o'>
            <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
            <detail>
                <contact callsign='BRAVO-2' endpoint='192.168.1.101:4242:tcp'/>
                <__group name='Red Team' role='Team Member'/>
                <takv os='31' version='5.4.0.16' device='SAMSUNG SM-G781U' platform='ATAK-CIV'/>
                <status battery='85'/>
                <track course='270.5' speed='15.2'/>
                <ditto a='pkAocCgkMCvR_e8DXneZfAsm6MYWwtINhKPmkHdwAvEwW4IKYmnh0' 
                       deviceName='DEVICE123' ip='192.168.1.101' version='AndJ4.10.2'/>
                <precisionlocation altsrc='GPS' geopointsrc='GPS'/>
                <uid Droid='BRAVO-2'/>
                <custom_data>
                    <field1>value1</field1>
                    <field2>value2</field2>
                    <nested>
                        <subfield1>subvalue1</subfield1>
                        <subfield2>subvalue2</subfield2>
                    </nested>
                </custom_data>
            </detail>
        </event>
        """;
    
    private static final String CHAT_COT_XML = """
        <?xml version='1.0' encoding='UTF-8'?>
        <event version='2.0' uid='CHAT-TEST-789' type='b-t-f' 
               time='2024-01-15T10:30:00.000Z' start='2024-01-15T10:30:00.000Z' 
               stale='2024-01-15T11:00:00.000Z' how='h-g-i-g-o'>
            <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
            <detail>
                <remarks sender='ALPHA-1' to='ALL' time='2024-01-15T10:30:00.000Z'>Test message</remarks>
                <__chat parent='ROOT' groupOwner='ALPHA-1' messageId='MSG-001' senderCallsign='ALPHA-1'>
                    <chatgrp uid0='BRAVO-2' uid1='CHARLIE-3'/>
                </__chat>
            </detail>
        </event>
        """;
    
    private static final String FILE_COT_XML = """
        <?xml version='1.0' encoding='UTF-8'?>
        <event version='2.0' uid='FILE-TEST-999' type='b-f-t-f' 
               time='2024-01-15T10:30:00.000Z' start='2024-01-15T10:30:00.000Z' 
               stale='2024-01-15T11:00:00.000Z' how='h-g-i-g-o'>
            <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
            <detail>
                <fileshare filename='test.pdf' senderUrl='http://192.168.1.100:8080/files/test.pdf' 
                          sizeInBytes='1048576' sha256hash='abc123def456' senderUid='ALPHA-1' 
                          senderCallsign='ALPHA-1' name='Test Document'/>
            </detail>
        </event>
        """;
    
    private CoTConverter converter;
    private CoTEvent simpleEvent;
    private CoTEvent complexEvent;
    private CoTEvent chatEvent;
    private CoTEvent fileEvent;
    private ObjectMapper objectMapper;
    
    @Setup
    public void setup() throws Exception {
        converter = new CoTConverter();
        objectMapper = new ObjectMapper();
        
        // Parse CoT events
        simpleEvent = converter.parseCoTXml(SIMPLE_COT_XML);
        complexEvent = converter.parseCoTXml(COMPLEX_COT_XML);
        chatEvent = converter.parseCoTXml(CHAT_COT_XML);
        fileEvent = converter.parseCoTXml(FILE_COT_XML);
    }
    
    @Benchmark
    public void parseSimpleXml(Blackhole blackhole) throws JAXBException {
        CoTEvent event = converter.parseCoTXml(SIMPLE_COT_XML);
        blackhole.consume(event);
    }
    
    @Benchmark
    public void parseComplexXml(Blackhole blackhole) throws JAXBException {
        CoTEvent event = converter.parseCoTXml(COMPLEX_COT_XML);
        blackhole.consume(event);
    }
    
    @Benchmark
    public void convertSimpleEventToDitto(Blackhole blackhole) {
        Object doc = converter.convertCoTEventToDocument(simpleEvent);
        blackhole.consume(doc);
    }
    
    @Benchmark
    public void convertComplexEventToDitto(Blackhole blackhole) {
        Object doc = converter.convertCoTEventToDocument(complexEvent);
        blackhole.consume(doc);
    }
    
    @Benchmark
    public void convertChatEventToDitto(Blackhole blackhole) {
        Object doc = converter.convertCoTEventToDocument(chatEvent);
        blackhole.consume(doc);
    }
    
    @Benchmark
    public void convertFileEventToDitto(Blackhole blackhole) {
        Object doc = converter.convertCoTEventToDocument(fileEvent);
        blackhole.consume(doc);
    }
    
    @Benchmark
    public void convertToXml(Blackhole blackhole) throws JAXBException {
        String xml = converter.marshalCoTEvent(simpleEvent);
        blackhole.consume(xml);
    }
    
    @Benchmark
    public void roundTripSimple(Blackhole blackhole) throws Exception {
        // XML -> CoT Event -> Ditto -> JSON
        CoTEvent parsed = converter.parseCoTXml(SIMPLE_COT_XML);
        Object dittoDoc = converter.convertCoTEventToDocument(parsed);
        String json = objectMapper.writeValueAsString(dittoDoc);
        blackhole.consume(json);
    }
    
    @Benchmark
    public void roundTripComplex(Blackhole blackhole) throws Exception {
        // XML -> CoT Event -> Ditto -> JSON
        CoTEvent parsed = converter.parseCoTXml(COMPLEX_COT_XML);
        Object dittoDoc = converter.convertCoTEventToDocument(parsed);
        String json = objectMapper.writeValueAsString(dittoDoc);
        blackhole.consume(json);
    }
    
    @Benchmark
    public void batchConversion(Blackhole blackhole) throws Exception {
        // Convert 100 events in batch
        for (int i = 0; i < 100; i++) {
            CoTEvent event = converter.parseCoTXml(SIMPLE_COT_XML);
            Object doc = converter.convertCoTEventToDocument(event);
            blackhole.consume(doc);
        }
    }
    
    @Benchmark
    public void serializeToJson(Blackhole blackhole) throws Exception {
        Object doc = converter.convertCoTEventToDocument(complexEvent);
        String json = objectMapper.writeValueAsString(doc);
        blackhole.consume(json);
    }
    
    @Benchmark
    public void deserializeFromJson(Blackhole blackhole) throws Exception {
        Object doc = converter.convertCoTEventToDocument(complexEvent);
        String json = objectMapper.writeValueAsString(doc);
        Map<String, Object> map = objectMapper.readValue(json, HashMap.class);
        blackhole.consume(map);
    }
    
    @Benchmark
    @Threads(4)
    public void concurrentParsing(Blackhole blackhole) throws Exception {
        // Test thread safety and concurrent performance
        CoTEvent event = converter.parseCoTXml(COMPLEX_COT_XML);
        Object doc = converter.convertCoTEventToDocument(event);
        blackhole.consume(doc);
    }
    
    /**
     * Main method to run benchmarks standalone.
     * Can be run with: java -cp <classpath> com.ditto.cot.performance.CoTPerformanceBenchmark
     */
    public static void main(String[] args) throws RunnerException {
        Options opt = new OptionsBuilder()
                .include(CoTPerformanceBenchmark.class.getSimpleName())
                .forks(1)
                .build();
        
        new Runner(opt).run();
    }
}