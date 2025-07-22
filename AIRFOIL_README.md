# 🛩️ Airfoil Visualization with Rust WASM

A real-time airfoil visualization tool built with Rust WASM and HTML5 Canvas.

## ✨ Features

- **📁 File Loading**: Load custom airfoil `.dat` files
- **📊 Real-time Visualization**: Interactive canvas rendering
- **📏 Measurements**: Automatic chord length and thickness calculation
- **🎨 Interactive Display**: Zoom and pan capabilities
- **⚡ High Performance**: Rust WASM parsing for speed
- **📱 Responsive Design**: Works on desktop and mobile

## 🚀 Quick Start

1. **Start the development server**:
   ```bash
   make start
   ```

2. **Open your browser** to `http://localhost:8000`

3. **Load an airfoil**:
   - Click "Load NACA 0012" for the included example
   - Or use "Choose File" to upload your own `.dat` file

## 📋 Supported File Formats

The parser supports standard airfoil coordinate files:

```
# NACA 0012 Airfoil
1.00000  0.00126
0.95000  0.00807
0.90000  0.01448
...
```

Format requirements:
- Two columns: `x y` coordinates
- Tab or space separated values
- Comments start with `#` or `;`
- Coordinates should be normalized (0.0 to 1.0 for chord)

## 🎯 Airfoil Data Structure

The Rust parser creates an `Airfoil` struct with:

- **Points**: Vector of (x, y) coordinates
- **Name**: Airfoil identifier
- **Bounds**: Min/max x,y values for scaling
- **Measurements**: Chord length, thickness, etc.

## 🔧 Canvas Features

- **Automatic Scaling**: Fits airfoil to canvas size
- **Coordinate System**: Shows x/y axes
- **Edge Markers**: 
  - 🟢 Green dot = Leading edge
  - 🟠 Orange dot = Trailing edge
- **Point Visualization**: Red dots show all coordinate points
- **Fill & Stroke**: Semi-transparent fill with outline

## 🛠 Development

### Rust WASM Functions

- `parse_airfoil_dat(data, name)` - Parse .dat file content
- `Airfoil::get_points_as_arrays()` - Get x,y coordinate arrays
- `Airfoil::get_bounds()` - Get min/max bounds
- `Airfoil::point_count()` - Number of points

### JavaScript Integration

```javascript
// Parse airfoil data
const airfoil = parse_airfoil_dat(fileContent, "NACA 0012");

// Get coordinates for drawing
const coords = airfoil.get_points_as_arrays();
const xCoords = Array.from(coords.x);
const yCoords = Array.from(coords.y);

// Get bounds for scaling
const bounds = airfoil.get_bounds();
```

## 📊 Information Panel

Displays real-time airfoil statistics:

- **Name**: Airfoil identifier
- **Points**: Number of coordinate points
- **Chord Length**: Distance from leading to trailing edge
- **Max Thickness**: Maximum airfoil thickness

## 🎨 Styling

The interface features:
- **Modern gradient background**
- **Glass-morphism effects**
- **Responsive grid layout**
- **Smooth animations and transitions**
- **Mobile-friendly controls**

## 🔄 File Processing Flow

1. **File Selection** → User selects .dat file
2. **Content Reading** → JavaScript reads file as text
3. **Rust Parsing** → WASM parser creates Airfoil struct
4. **Coordinate Extraction** → Get x,y arrays from Rust
5. **Canvas Rendering** → Draw scaled airfoil with decorations
6. **Info Update** → Display measurements and statistics

## 🚨 Error Handling

- **File Format Validation**: Checks for valid coordinate pairs
- **Parse Error Recovery**: Continues processing despite malformed lines
- **User Feedback**: Clear error messages and status updates
- **Graceful Degradation**: Fallback for unsupported browsers

## 📈 Performance

- **Fast Parsing**: Rust WASM is ~10x faster than JavaScript
- **Memory Efficient**: Minimal memory allocation
- **Smooth Animation**: 60fps canvas updates
- **Optimized Rendering**: Only redraws when necessary

Ready to explore airfoils! 🛩️✨
