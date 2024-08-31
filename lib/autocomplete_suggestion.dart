import 'package:flutter/widgets.dart';

class AutocompleteSuggestion extends StatelessWidget {
  final String suggestion;

  const AutocompleteSuggestion({super.key, required this.suggestion});
  
  @override
  Widget build(BuildContext context) {
    return Container(
              decoration: const BoxDecoration(
                border: Border(bottom: BorderSide(color: Color.fromARGB(194, 85, 85, 85))),
              ),
              padding: const EdgeInsets.all(8),
              child: Text(
                suggestion,
                style: const TextStyle(
                  color: Color.fromARGB(255, 255, 255, 255),
                  fontSize: 16,
                ),
              ),
            );
  }
}