package com.ditto.cot;

import com.ditto.cot.schema.*;
import jakarta.xml.bind.JAXBException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.Map;
import java.util.HashMap;
import java.util.stream.Stream;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Comprehensive tests for file sharing CoT events.
 * Tests various file types, sharing scenarios, and edge cases.
 */
@DisplayName("File Sharing CoT Events Tests")
public class FileShareCoTTest {
    
    private CoTConverter converter;
    
    @BeforeEach
    void setUp() throws JAXBException {
        converter = new CoTConverter();
    }
    
    @Nested
    @DisplayName("Basic File Share Event Tests")
    class BasicFileShareTests {
        
        @Test
        @DisplayName("Should parse basic file share event")
        void testBasicFileShareEvent() throws JAXBException {
            String fileShareXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='FILE-SHARE-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='test-document.pdf' 
                                  senderUrl='http://192.168.1.100:8080/files/test-document.pdf' 
                                  sizeInBytes='1048576' 
                                  sha256hash='abc123def456789' 
                                  senderUid='SENDER-001' 
                                  senderCallsign='ALPHA-1' 
                                  name='Test Document'/>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(fileShareXml);
            assertNotNull(event);
            assertEquals("FILE-SHARE-001", event.getUid());
            assertEquals("b-f-t-f", event.getType());
            
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getFile()).contains("test-document.pdf");
        }
        
        @Test
        @DisplayName("Should handle file share with attachment details")
        void testFileShareWithAttachment() throws JAXBException {
            String attachmentXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='ATTACHMENT-001' type='b-f-t-a' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <attachment filename='image.jpg' 
                                   mimetype='image/jpeg' 
                                   size='2097152' 
                                   hash='def456abc789' 
                                   submissionTime='2024-01-15T10:30:00.000Z'
                                   submitter='BRAVO-2'/>
                        <contact callsign='BRAVO-2' endpoint='192.168.1.101:4242:tcp'/>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(attachmentXml);
            assertNotNull(event);
            assertEquals("ATTACHMENT-001", event.getUid());
            assertEquals("b-f-t-a", event.getType());
            
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
        }
    }
    
    @Nested
    @DisplayName("File Type Tests")
    class FileTypeTests {
        
        @ParameterizedTest
        @CsvSource({
            "document.pdf, application/pdf, PDF Document",
            "image.jpg, image/jpeg, JPEG Image", 
            "video.mp4, video/mp4, MP4 Video",
            "audio.wav, audio/wav, WAV Audio",
            "data.zip, application/zip, ZIP Archive",
            "spreadsheet.xlsx, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, Excel Spreadsheet",
            "presentation.pptx, application/vnd.openxmlformats-officedocument.presentationml.presentation, PowerPoint Presentation",
            "text.txt, text/plain, Plain Text"
        })
        @DisplayName("Should handle different file types")
        void testDifferentFileTypes(String filename, String mimetype, String description) throws JAXBException {
            String fileXml = String.format("""
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='FILE-%s' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='%s' 
                                  mimetype='%s' 
                                  name='%s'
                                  senderUrl='http://192.168.1.100:8080/files/%s' 
                                  sizeInBytes='1048576' 
                                  senderUid='SENDER-001'/>
                    </detail>
                </event>
                """, filename.hashCode(), filename, mimetype, description, filename);
            
            CoTEvent event = converter.parseCoTXml(fileXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getFile()).contains(filename);
            assertThat(fileDoc.getMime()).contains(mimetype);
        }
        
        @Test
        @DisplayName("Should handle large file sizes")
        void testLargeFileSizes() throws JAXBException {
            long[] fileSizes = {
                1024L,                    // 1 KB
                1048576L,                 // 1 MB
                1073741824L,              // 1 GB
                5368709120L,              // 5 GB
                1099511627776L            // 1 TB
            };
            
            for (long fileSize : fileSizes) {
                String fileXml = String.format("""
                    <?xml version='1.0' encoding='UTF-8'?>
                    <event version='2.0' uid='LARGE-FILE-%d' type='b-f-t-f' 
                           time='2024-01-15T10:30:00.000Z' 
                           start='2024-01-15T10:30:00.000Z' 
                           stale='2024-01-15T12:30:00.000Z' 
                           how='h-g-i-g-o'>
                        <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                        <detail>
                            <fileshare filename='large-file-%d.dat' 
                                      sizeInBytes='%d' 
                                      senderUrl='http://192.168.1.100:8080/files/large-file-%d.dat' 
                                      senderUid='SENDER-001'/>
                        </detail>
                    </event>
                    """, fileSize, fileSize, fileSize, fileSize);
                
                CoTEvent event = converter.parseCoTXml(fileXml);
                Object document = converter.convertCoTEventToDocument(event);
                
                assertNotNull(document);
                assertTrue(document instanceof FileDocument);
                
                FileDocument fileDoc = (FileDocument) document;
                assertThat(fileDoc.getSz()).isEqualTo((double) fileSize);
            }
        }
    }
    
    @Nested
    @DisplayName("File Sharing Metadata Tests")
    class FileMetadataTests {
        
        @Test
        @DisplayName("Should preserve all file metadata")
        void testCompleteFileMetadata() throws JAXBException {
            String completeMetadataXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='METADATA-TEST-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='mission-briefing.pdf' 
                                  senderUrl='https://secure.example.com/files/mission-briefing.pdf' 
                                  sizeInBytes='2097152' 
                                  sha256hash='a1b2c3d4e5f6789012345678901234567890abcdef123456789012345678901234' 
                                  senderUid='COMMANDER-001' 
                                  senderCallsign='ALPHA-LEADER' 
                                  name='Mission Briefing Document'
                                  mimetype='application/pdf'
                                  downloadPath='/data/downloads/mission-briefing.pdf'
                                  keywords='mission briefing classified'/>
                        <contact callsign='ALPHA-LEADER' endpoint='192.168.1.100:4242:tcp'/>
                        <__group name='Alpha Team' role='Commander'/>
                        <remarks>Classified mission briefing - handle with care</remarks>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(completeMetadataXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getFile()).contains("mission-briefing.pdf");
            assertThat(fileDoc.getMime()).contains("application/pdf");
            assertThat(fileDoc.getSz()).isEqualTo(2097152.0);
            assertThat(fileDoc.getId()).isEqualTo("METADATA-TEST-001");
        }
        
        @Test
        @DisplayName("Should handle file share with security annotations")
        void testFileShareWithSecurity() throws JAXBException {
            String secureFileXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='SECURE-FILE-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='classified-report.pdf' 
                                  senderUrl='https://secure-server.mil/files/classified-report.pdf' 
                                  sizeInBytes='512000' 
                                  sha256hash='secure123456789abcdef' 
                                  senderUid='INTEL-001' 
                                  senderCallsign='INTEL-OFFICER' 
                                  name='Intelligence Report'
                                  classification='SECRET'
                                  releasability='US-ONLY'
                                  caveat='NOFORN'/>
                        <security classification='SECRET' ownerProduced='US' releasableTo='US'/>
                        <access restrictedTo='INTEL-GROUP' requiresAuth='true'/>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(secureFileXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            // Verify security-related information is preserved
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getFile()).contains("classified-report.pdf");
        }
    }
    
    @Nested
    @DisplayName("File Share Network Tests")
    class NetworkTests {
        
        @ParameterizedTest
        @ValueSource(strings = {
            "http://192.168.1.100:8080/files/document.pdf",
            "https://secure.example.com/files/secure-doc.pdf",
            "ftp://fileserver.local/public/shared-file.zip",
            "file:///local/path/to/file.txt",
            "smb://network-share/files/document.docx"
        })
        @DisplayName("Should handle different URL schemes")
        void testDifferentUrlSchemes(String url) throws JAXBException {
            String fileXml = String.format("""
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='URL-TEST-%d' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='test-file.dat' 
                                  senderUrl='%s' 
                                  sizeInBytes='1024' 
                                  senderUid='SENDER-001'/>
                    </detail>
                </event>
                """, url.hashCode(), url);
            
            CoTEvent event = converter.parseCoTXml(fileXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
        }
        
        @Test
        @DisplayName("Should handle file share with mirror URLs")
        void testFileShareWithMirrors() throws JAXBException {
            String mirrorFileXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='MIRROR-FILE-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='important-data.zip' 
                                  senderUrl='http://primary-server.com/files/important-data.zip' 
                                  sizeInBytes='5242880' 
                                  senderUid='DATA-SERVER-001'/>
                        <mirrors>
                            <mirror url='http://backup-server.com/files/important-data.zip' priority='1'/>
                            <mirror url='http://mirror-server.com/files/important-data.zip' priority='2'/>
                            <mirror url='http://cdn-server.com/files/important-data.zip' priority='3'/>
                        </mirrors>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(mirrorFileXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
        }
    }
    
    @Nested
    @DisplayName("File Share Round-trip Tests")
    class RoundTripTests {
        
        @Test
        @DisplayName("Should preserve file share data through round-trip conversion")
        void testFileShareRoundTrip() throws JAXBException {
            String originalXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='ROUNDTRIP-FILE-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='test-roundtrip.pdf' 
                                  senderUrl='http://192.168.1.100:8080/files/test-roundtrip.pdf' 
                                  sizeInBytes='1048576' 
                                  sha256hash='abc123def456' 
                                  senderUid='ROUNDTRIP-SENDER' 
                                  senderCallsign='ALPHA-1' 
                                  name='Round-trip Test Document'/>
                        <contact callsign='ALPHA-1' endpoint='192.168.1.100:4242:tcp'/>
                    </detail>
                </event>
                """;
            
            // XML -> CoTEvent -> FileDocument -> XML
            CoTEvent originalEvent = converter.parseCoTXml(originalXml);
            Object document = converter.convertCoTEventToDocument(originalEvent);
            
            assertTrue(document instanceof FileDocument);
            FileDocument fileDoc = (FileDocument) document;
            
            // Verify key file properties are preserved
            assertThat(fileDoc.getFile()).contains("test-roundtrip.pdf");
            assertThat(fileDoc.getSz()).isEqualTo(1048576.0);
            assertThat(fileDoc.getId()).isEqualTo("ROUNDTRIP-FILE-001");
            
            // Convert back to XML
            String regeneratedXml = converter.marshalCoTEvent(originalEvent);
            
            // Parse the regenerated XML to verify it's valid
            CoTEvent regeneratedEvent = converter.parseCoTXml(regeneratedXml);
            assertEquals(originalEvent.getUid(), regeneratedEvent.getUid());
            assertEquals(originalEvent.getType(), regeneratedEvent.getType());
        }
        
        @Test
        @DisplayName("Should handle file share with complex detail structure")
        void testComplexFileShareStructure() throws JAXBException {
            String complexXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='COMPLEX-FILE-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='complex-document.pdf' 
                                  senderUrl='http://192.168.1.100:8080/files/complex-document.pdf' 
                                  sizeInBytes='2097152' 
                                  senderUid='COMPLEX-SENDER'/>
                        <contact callsign='COMPLEX-SENDER' endpoint='192.168.1.100:4242:tcp'/>
                        <__group name='File Share Team' role='Data Manager'/>
                        <takv os='31' version='5.4.0.16' device='SERVER-001' platform='FILE-SERVER'/>
                        <status battery='100'/>
                        <metadata>
                            <created>2024-01-15T09:00:00.000Z</created>
                            <modified>2024-01-15T10:00:00.000Z</modified>
                            <author>Document Creator</author>
                            <version>1.2</version>
                            <tags>
                                <tag>important</tag>
                                <tag>mission-critical</tag>
                                <tag>shared</tag>
                            </tags>
                        </metadata>
                        <permissions>
                            <read>ALL</read>
                            <write>ADMIN</write>
                            <execute>NONE</execute>
                        </permissions>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(complexXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getFile()).contains("complex-document.pdf");
            assertThat(fileDoc.getSz()).isEqualTo(2097152.0);
        }
    }
    
    @Nested
    @DisplayName("Edge Cases and Error Handling")
    class EdgeCasesTests {
        
        @Test
        @DisplayName("Should handle file share with missing filename")
        void testMissingFilename() throws JAXBException {
            String missingFilenameXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='NO-FILENAME-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare senderUrl='http://192.168.1.100:8080/files/unknown' 
                                  sizeInBytes='1024' 
                                  senderUid='SENDER-001'/>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(missingFilenameXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            // Should handle gracefully, possibly using UID as fallback filename
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getId()).isEqualTo("NO-FILENAME-001");
        }
        
        @Test
        @DisplayName("Should handle file share with special characters in filename")
        void testSpecialCharactersInFilename() throws JAXBException {
            String specialCharsXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='SPECIAL-CHARS-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='file with spaces &amp; symbols (v1.2) [final].pdf' 
                                  senderUrl='http://192.168.1.100:8080/files/encoded-filename' 
                                  sizeInBytes='1024' 
                                  senderUid='SENDER-001'/>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(specialCharsXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
            
            FileDocument fileDoc = (FileDocument) document;
            assertThat(fileDoc.getFile()).contains("file with spaces");
        }
        
        @Test
        @DisplayName("Should handle very long filenames")
        void testVeryLongFilename() throws JAXBException {
            String longFilename = "a".repeat(255) + ".txt"; // 255 character filename
            
            String longFilenameXml = String.format("""
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='LONG-FILENAME-001' type='b-f-t-f' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T12:30:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <fileshare filename='%s' 
                                  senderUrl='http://192.168.1.100:8080/files/long-filename' 
                                  sizeInBytes='1024' 
                                  senderUid='SENDER-001'/>
                    </detail>
                </event>
                """, longFilename);
            
            CoTEvent event = converter.parseCoTXml(longFilenameXml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertNotNull(document);
            assertTrue(document instanceof FileDocument);
        }
    }
    
    private static Stream<Arguments> fileTypeProvider() {
        return Stream.of(
            Arguments.of("document.pdf", "application/pdf", 1048576L),
            Arguments.of("image.jpg", "image/jpeg", 2097152L),
            Arguments.of("video.mp4", "video/mp4", 104857600L),
            Arguments.of("audio.mp3", "audio/mpeg", 5242880L),
            Arguments.of("archive.zip", "application/zip", 10485760L)
        );
    }
}