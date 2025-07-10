import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

/// Home screen example demonstrating Flutter best practices
/// 
/// This screen shows:
/// - Proper state management integration
/// - Responsive design patterns  
/// - Material Design 3 components
/// - Error handling and loading states
class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Home'),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      body: Consumer<DataProvider>(
        builder: (context, dataProvider, child) {
          return RefreshIndicator(
            onRefresh: () => dataProvider.refreshData(),
            child: _buildBody(context, dataProvider),
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _handleAddAction(context),
        tooltip: 'Add Item',
        child: const Icon(Icons.add),
      ),
    );
  }

  Widget _buildBody(BuildContext context, DataProvider dataProvider) {
    if (dataProvider.isLoading) {
      return const Center(
        child: CircularProgressIndicator(),
      );
    }

    if (dataProvider.hasError) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 64,
              color: Theme.of(context).colorScheme.error,
            ),
            const SizedBox(height: 16),
            Text(
              'Something went wrong',
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            const SizedBox(height: 8),
            Text(
              dataProvider.errorMessage ?? 'Unknown error occurred',
              style: Theme.of(context).textTheme.bodyMedium,
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            FilledButton(
              onPressed: () => dataProvider.refreshData(),
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    final items = dataProvider.items;
    if (items.isEmpty) {
      return const Center(
        child: Text('No items found'),
      );
    }

    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: items.length,
      itemBuilder: (context, index) {
        final item = items[index];
        return Card(
          margin: const EdgeInsets.only(bottom: 8),
          child: ListTile(
            title: Text(item.title),
            subtitle: Text(item.description),
            trailing: IconButton(
              icon: const Icon(Icons.arrow_forward_ios),
              onPressed: () => _navigateToDetail(context, item),
            ),
          ),
        );
      },
    );
  }

  void _handleAddAction(BuildContext context) {
    // Navigate to add item screen
    Navigator.of(context).pushNamed('/add');
  }

  void _navigateToDetail(BuildContext context, dynamic item) {
    Navigator.of(context).pushNamed(
      '/detail',
      arguments: item,
    );
  }
}