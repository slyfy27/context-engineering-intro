## FEATURE:

- Flutter mobile app with a weather dashboard that displays current weather and 7-day forecast.
- Clean and modern Material Design 3 UI with dark/light theme support.
- Location-based weather using device GPS or manual city search.
- State management using Provider or Riverpod for reactive data updates.
- Offline support with local caching of weather data.
- Settings screen for temperature units (Celsius/Fahrenheit) and theme preferences.

## EXAMPLES:

In the `examples/` folder, there is a README for you to read to understand what the example is all about and also how to structure your own README when you create documentation for the above feature.

- `examples/screens/` - use these as templates for creating clean, responsive Flutter screens
- `examples/widgets/` - reusable UI components following Material Design 3 principles
- `examples/services/` - API service patterns for HTTP requests and data handling
- `examples/models/` - data model examples with proper JSON serialization
- `examples/providers/` - state management patterns using Provider/Riverpod
- `examples/utils/` - helper functions for common Flutter operations

Don't copy any of these examples directly, it is for a different project entirely. But use this as inspiration and for best practices.

## DOCUMENTATION:

Flutter documentation: https://docs.flutter.dev/
Material Design 3: https://m3.material.io/
OpenWeatherMap API: https://openweathermap.org/api
Provider package: https://pub.dev/packages/provider
HTTP package: https://pub.dev/packages/http

## OTHER CONSIDERATIONS:

- Include a .env.example file for API keys, use flutter_dotenv for environment variables
- Include a comprehensive README with setup instructions including API key configuration
- Add proper error handling for network failures and invalid locations
- Implement pull-to-refresh functionality for weather updates
- Ensure app works on both Android and iOS with proper platform-specific styling
- Include proper loading states and error messages for better UX
- Add proper app icons and splash screen
- Follow Flutter best practices for performance optimization
