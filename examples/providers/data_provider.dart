import 'package:flutter/foundation.dart';
import '../services/api_service.dart';
import '../models/data_model.dart';

/// Data provider example demonstrating proper state management patterns
/// 
/// This provider shows:
/// - Proper async state handling
/// - Error management
/// - Loading states
/// - Data caching patterns
class DataProvider extends ChangeNotifier {
  final ApiService _apiService;
  
  // Private state variables
  List<DataModel> _items = [];
  bool _isLoading = false;
  String? _errorMessage;
  DateTime? _lastFetchTime;

  // Constructor
  DataProvider({required ApiService apiService}) : _apiService = apiService;

  // Public getters
  List<DataModel> get items => List.unmodifiable(_items);
  bool get isLoading => _isLoading;
  bool get hasError => _errorMessage != null;
  String? get errorMessage => _errorMessage;
  bool get needsRefresh {
    if (_lastFetchTime == null) return true;
    return DateTime.now().difference(_lastFetchTime!).inMinutes > 5;
  }

  /// Fetch data from API with proper error handling
  Future<void> fetchData() async {
    if (_isLoading) return; // Prevent concurrent requests

    _setLoading(true);
    _clearError();

    try {
      final data = await _apiService.fetchItems();
      _items = data;
      _lastFetchTime = DateTime.now();
      
      // Log success for debugging
      if (kDebugMode) {
        print('Successfully fetched ${data.length} items');
      }
    } catch (error) {
      _handleError(error);
    } finally {
      _setLoading(false);
    }
  }

  /// Refresh data (public method for UI to call)
  Future<void> refreshData() async {
    _lastFetchTime = null; // Force refresh
    await fetchData();
  }

  /// Add a new item
  Future<void> addItem(DataModel item) async {
    _setLoading(true);
    _clearError();

    try {
      final newItem = await _apiService.createItem(item);
      _items = [..._items, newItem];
      
      if (kDebugMode) {
        print('Successfully added item: ${newItem.id}');
      }
    } catch (error) {
      _handleError(error);
    } finally {
      _setLoading(false);
    }
  }

  /// Update an existing item
  Future<void> updateItem(DataModel item) async {
    _setLoading(true);
    _clearError();

    try {
      final updatedItem = await _apiService.updateItem(item);
      final index = _items.indexWhere((i) => i.id == item.id);
      
      if (index != -1) {
        _items = [
          ..._items.sublist(0, index),
          updatedItem,
          ..._items.sublist(index + 1),
        ];
      }
      
      if (kDebugMode) {
        print('Successfully updated item: ${updatedItem.id}');
      }
    } catch (error) {
      _handleError(error);
    } finally {
      _setLoading(false);
    }
  }

  /// Delete an item
  Future<void> deleteItem(String itemId) async {
    _setLoading(true);
    _clearError();

    try {
      await _apiService.deleteItem(itemId);
      _items = _items.where((item) => item.id != itemId).toList();
      
      if (kDebugMode) {
        print('Successfully deleted item: $itemId');
      }
    } catch (error) {
      _handleError(error);
    } finally {
      _setLoading(false);
    }
  }

  /// Search items locally (no API call)
  List<DataModel> searchItems(String query) {
    if (query.isEmpty) return items;
    
    return _items.where((item) {
      return item.title.toLowerCase().contains(query.toLowerCase()) ||
             item.description.toLowerCase().contains(query.toLowerCase());
    }).toList();
  }

  /// Clear all data (useful for logout)
  void clearData() {
    _items = [];
    _lastFetchTime = null;
    _clearError();
    notifyListeners();
  }

  // Private helper methods
  void _setLoading(bool loading) {
    if (_isLoading != loading) {
      _isLoading = loading;
      notifyListeners();
    }
  }

  void _clearError() {
    if (_errorMessage != null) {
      _errorMessage = null;
      notifyListeners();
    }
  }

  void _handleError(dynamic error) {
    _errorMessage = error.toString();
    
    // Log error for debugging
    if (kDebugMode) {
      print('DataProvider error: $error');
    }
    
    notifyListeners();
  }
}