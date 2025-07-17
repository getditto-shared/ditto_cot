package com.ditto.cot;

/**
 * Factory class for creating the appropriate CoT converter based on the runtime environment.
 * Automatically detects whether JAXB is available and chooses the appropriate implementation.
 */
public class CoTConverterFactory {
    
    private static final boolean IS_JAXB_AVAILABLE = isJaxbAvailable();
    
    /**
     * Create a CoT converter appropriate for the current runtime environment.
     * On Android or when JAXB is not available, returns AndroidCoTConverter.
     * On regular Java environments with JAXB, returns CoTConverter.
     */
    public static Object createConverter() {
        if (IS_JAXB_AVAILABLE) {
            try {
                // Use JAXB-based converter for regular Java environments
                return new CoTConverter();
            } catch (Exception e) {
                // If JAXB fails for any reason, fall back to Android converter
                return new AndroidCoTConverter();
            }
        } else {
            // Use Android-compatible converter
            return new AndroidCoTConverter();
        }
    }
    
    /**
     * Create an Android-compatible converter explicitly.
     * This is useful when you want to force the use of the Android implementation
     * even in environments where JAXB is available.
     */
    public static AndroidCoTConverter createAndroidConverter() {
        return new AndroidCoTConverter();
    }
    
    /**
     * Create a JAXB-based converter explicitly.
     * This will throw an exception if JAXB is not available.
     */
    public static CoTConverter createJaxbConverter() throws Exception {
        if (!IS_JAXB_AVAILABLE) {
            throw new UnsupportedOperationException("JAXB is not available in this environment");
        }
        return new CoTConverter();
    }
    
    /**
     * Check if JAXB is available in the current runtime environment.
     */
    private static boolean isJaxbAvailable() {
        try {
            // Try to load the JAXB context class
            Class.forName("jakarta.xml.bind.JAXBContext");
            return true;
        } catch (ClassNotFoundException e) {
            try {
                // Try the older javax.xml.bind package
                Class.forName("javax.xml.bind.JAXBContext");
                return true;
            } catch (ClassNotFoundException ex) {
                return false;
            }
        }
    }
    
    /**
     * Check if the current runtime environment is Android.
     */
    public static boolean isAndroid() {
        return System.getProperty("java.vendor").toLowerCase().contains("android") ||
               System.getProperty("java.vm.name").toLowerCase().contains("android") ||
               System.getProperty("java.runtime.name").toLowerCase().contains("android");
    }
    
    
    /**
     * Get information about the current runtime environment.
     */
    public static String getRuntimeInfo() {
        StringBuilder info = new StringBuilder();
        info.append("Java Version: ").append(System.getProperty("java.version")).append("\n");
        info.append("Java Vendor: ").append(System.getProperty("java.vendor")).append("\n");
        info.append("Java Runtime: ").append(System.getProperty("java.runtime.name")).append("\n");
        info.append("Java VM: ").append(System.getProperty("java.vm.name")).append("\n");
        info.append("Is Android: ").append(isAndroid()).append("\n");
        info.append("JAXB Available: ").append(isJaxbAvailable()).append("\n");
        return info.toString();
    }
}