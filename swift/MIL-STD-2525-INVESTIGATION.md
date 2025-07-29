# MIL-STD-2525 Iconography Implementation Investigation

## Overview
This document investigates what it would take to incorporate MIL-STD-2525 military symbology into the Swift CoT app.

## Current State
The app currently uses simple colored circles with SF Symbols (Apple's system icons) to represent different entity types:
- Friendly units: Green circle with shield icon
- Hostile units: Red circle 
- Neutral units: Yellow circle
- Unknown units: Blue circle
- Tracks: Cyan/Mint circle with location icon

## MIL-STD-2525 Background

### Symbol Components
MIL-STD-2525 symbols consist of:
1. **Frame** - Shape indicating affiliation (friendly=rectangle, hostile=diamond, neutral=square, unknown=cloverleaf)
2. **Fill** - Color coding (blue/green=friendly, red=hostile, yellow=neutral)
3. **Icon** - Central pictograph indicating unit type/function
4. **Modifiers** - Text and graphic modifiers around the symbol

### Symbol Identification Code (SIDC)
- **2525C and earlier**: 15-character alphanumeric code
- **2525D**: 20 or 30-digit numeric code

### CoT to MIL-STD-2525 Mapping
CoT type codes partially map to MIL-STD-2525:
- Example: `a-f-G-U-C` = atoms-friendly-Ground-Unit-Combat
- First position: Event category (a=atoms)
- Second position: Affiliation (f=friendly)
- Third position: Battle dimension (G=ground)
- Remaining: Function codes

## Implementation Options

### 1. Use Existing Libraries

#### A. Nobori C++ SDK
- **Pros**: High performance, macOS support, complete implementation
- **Cons**: Requires C++ integration, potential licensing costs
- **Integration**: Objective-C++ bridging layer

#### B. milsymbol JavaScript Library
- **Pros**: Open source (MIT), well-maintained, complete
- **Cons**: JavaScript overhead, not native
- **Integration**: 
  - Option 1: WKWebView with JavaScript interface
  - Option 2: JavaScriptCore for headless rendering
  - Option 3: Port to Swift

#### C. Native Swift Implementation
- **Pros**: Native performance, full control, no dependencies
- **Cons**: Significant development effort
- **Approach**: 
  - Parse CoT type codes to extract components
  - Map to SIDC codes
  - Render using Core Graphics or SVG

### 2. Rendering Approaches

#### A. Pre-rendered Images
- Generate all possible symbols offline
- Bundle as assets or download on demand
- **Pros**: Simple, fast rendering
- **Cons**: Large app size, limited customization

#### B. Dynamic Vector Rendering
- Use Core Graphics or SVG to draw symbols
- **Pros**: Small size, infinite scaling, customizable
- **Cons**: Complex implementation

#### C. Hybrid Approach
- Pre-render common symbols
- Dynamic rendering for rare/custom symbols
- Best balance of performance and flexibility

## Recommended Implementation Plan

### Phase 1: Basic Symbol Support (2-3 weeks)
1. Create Swift data model for MIL-STD-2525 symbols
2. Implement CoT type to SIDC converter
3. Add basic frame rendering (rectangle, diamond, square)
4. Use SF Symbols as temporary icons

### Phase 2: Full Symbol Library (4-6 weeks)
1. Port milsymbol geometry to Swift/Core Graphics
2. Implement all standard icons
3. Add text modifiers support
4. Cache rendered symbols

### Phase 3: Advanced Features (2-3 weeks)
1. Add tactical graphics support
2. Implement symbol modifiers
3. Add animation for moving symbols
4. Performance optimization

## Code Architecture

```swift
// Symbol model
struct MilStd2525Symbol {
    let sidc: String
    let affiliation: Affiliation
    let battleDimension: BattleDimension
    let function: String
    let modifiers: [String: String]
}

// Renderer protocol
protocol SymbolRenderer {
    func render(symbol: MilStd2525Symbol, size: CGSize) -> UIImage
}

// CoT converter
class CoTToMilStdConverter {
    func convert(cotType: String) -> MilStd2525Symbol? {
        // Parse CoT type and map to SIDC
    }
}
```

## Integration with Current App

1. Replace `EventAnnotation` view with new `MilStdSymbolView`
2. Add toggle in settings for symbol style (simple vs MIL-STD)
3. Update `CoTEventModel` to include SIDC data
4. Cache rendered symbols for performance

## Testing Requirements

1. Validate CoT to SIDC conversion accuracy
2. Test all affiliation/dimension combinations
3. Performance testing with 1000+ symbols
4. Cross-reference with ATAK display

## Estimated Timeline

- **Minimum Viable**: 3-4 weeks (basic shapes and colors)
- **Full Implementation**: 8-10 weeks (complete symbol set)
- **Production Ready**: 12-14 weeks (with optimization and testing)

## Risks and Considerations

1. **Complexity**: MIL-STD-2525D has thousands of possible symbols
2. **Performance**: Rendering complex symbols on mobile devices
3. **Accuracy**: Ensuring correct interpretation of standards
4. **Maintenance**: Keeping up with standard updates
5. **File Size**: Symbol assets or rendering code could significantly increase app size

## Recommendation

Start with a phased approach:
1. Implement basic frame shapes and colors based on CoT affiliation
2. Use a subset of common symbols (top 50-100)
3. Consider licensing Nobori SDK for production use
4. Alternatively, create Swift port of milsymbol core functionality

This would provide military-standard symbols while managing complexity and development time.