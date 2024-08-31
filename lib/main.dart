// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// // The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.
//
// // import 'package:flutter/material.dart';
// //
// // void main() {
// //   runApp(const MyApp());
// // }
// //
// // class MyApp extends StatelessWidget {
// //   const MyApp({super.key});
// //
// //   // This widget is the root of your application.
// //   @override
// //   Widget build(BuildContext context) {
// //     return MaterialApp(
// //       title: 'Flutter Demo',
// //       theme: ThemeData(
// //         // This is the theme of your application.
// //         //
// //         // TRY THIS: Try running your application with "flutter run". You'll see
// //         // the application has a purple toolbar. Then, without quitting the app,
// //         // try changing the seedColor in the colorScheme below to Colors.green
// //         // and then invoke "hot reload" (save your changes or press the "hot
// //         // reload" button in a Flutter-supported IDE, or press "r" if you used
// //         // the command line to start the app).
// //         //
// //         // Notice that the counter didn't reset back to zero; the application
// //         // state is not lost during the reload. To reset the state, use hot
// //         // restart instead.
// //         //
// //         // This works for code too, not just values: Most code changes can be
// //         // tested with just a hot reload.
// //         colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
// //         useMaterial3: true,
// //       ),
// //       home: const MyHomePage(title: 'Flutter Demo Home Page'),
// //     );
// //   }
// // }
// //
// // class MyHomePage extends StatefulWidget {
// //   const MyHomePage({super.key, required this.title});
// //
// //   // This widget is the home page of your application. It is stateful, meaning
// //   // that it has a State object (defined below) that contains fields that affect
// //   // how it looks.
// //
// //   // This class is the configuration for the state. It holds the values (in this
// //   // case the title) provided by the parent (in this case the App widget) and
// //   // used by the build method of the State. Fields in a Widget subclass are
// //   // always marked "final".
// //
// //   final String title;
// //
// //   @override
// //   State<MyHomePage> createState() => _MyHomePageState();
// // }
// //
// // class _MyHomePageState extends State<MyHomePage> {
// //   int _counter = 0;
// //
// //   void _incrementCounter() {
// //     setState(() {
// //       // This call to setState tells the Flutter framework that something has
// //       // changed in this State, which causes it to rerun the build method below
// //       // so that the display can reflect the updated values. If we changed
// //       // _counter without calling setState(), then the build method would not be
// //       // called again, and so nothing would appear to happen.
// //       _counter++;
// //     });
// //   }
// //
// //   @override
// //   Widget build(BuildContext context) {
// //     // This method is rerun every time setState is called, for instance as done
// //     // by the _incrementCounter method above.
// //     //
// //     // The Flutter framework has been optimized to make rerunning build methods
// //     // fast, so that you can just rebuild anything that needs updating rather
// //     // than having to individually change instances of widgets.
// //     return Scaffold(
// //       appBar: AppBar(
// //         // TRY THIS: Try changing the color here to a specific color (to
// //         // Colors.amber, perhaps?) and trigger a hot reload to see the AppBar
// //         // change color while the other colors stay the same.
// //         backgroundColor: Theme.of(context).colorScheme.inversePrimary,
// //         // Here we take the value from the MyHomePage object that was created by
// //         // the App.build method, and use it to set our appbar title.
// //         title: Text(widget.title),
// //       ),
// //       body: Center(
// //         // Center is a layout widget. It takes a single child and positions it
// //         // in the middle of the parent.
// //         child: Column(
// //           // Column is also a layout widget. It takes a list of children and
// //           // arranges them vertically. By default, it sizes itself to fit its
// //           // children horizontally, and tries to be as tall as its parent.
// //           //
// //           // Column has various properties to control how it sizes itself and
// //           // how it positions its children. Here we use mainAxisAlignment to
// //           // center the children vertically; the main axis here is the vertical
// //           // axis because Columns are vertical (the cross axis would be
// //           // horizontal).
// //           //
// //           // TRY THIS: Invoke "debug painting" (choose the "Toggle Debug Paint"
// //           // action in the IDE, or press "p" in the console), to see the
// //           // wireframe for each widget.
// //           mainAxisAlignment: MainAxisAlignment.center,
// //           children: <Widget>[
// //             const Text(
// //               'You have pushed the button this many times:',
// //             ),
// //             Text(
// //               '$_counter',
// //               style: Theme.of(context).textTheme.headlineMedium,
// //             ),
// //           ],
// //         ),
// //       ),
// //       floatingActionButton: FloatingActionButton(
// //         onPressed: _incrementCounter,
// //         tooltip: 'Increment',
// //         child: const Icon(Icons.add),
// //       ), // This trailing comma makes auto-formatting nicer for build methods.
// //     );
// //   }
// // }
// //
//
// import 'package:flutter/material.dart';
// import 'package:nofi/src/rust/api/simple.dart';
// import 'package:nofi/src/rust/frb_generated.dart';
//
// Future<void> main() async {
//   await RustLib.init();
//   runApp(const MyApp());
// }
//
// class MyApp extends StatelessWidget {
//   const MyApp({super.key});
//
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       home: Scaffold(
//         appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
//         body: Center(
//           child: Text(
//              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
//         ),
//       ),
//     );
//   }
// }
//


import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:nofi2/src/rust/frb_generated.dart';
import 'package:nofi2/src/rust/nofi/eval.dart';
import 'package:nofi2/autocomplete_suggestion.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {

  BoxDecoration wrapper = BoxDecoration(
    color: const Color.fromARGB(255, 43, 27, 12),
    border: Border.all(color: const Color.fromARGB(255, 230, 184, 83), width: 3),
  );

  BoxDecoration altWrapper = BoxDecoration(
    color: const Color.fromARGB(255, 31, 19, 9),
    border: Border.all(color: const Color.fromARGB(255, 230, 184, 83), width: 3),
  );

  TextEditingController textcontroller = TextEditingController();

  double imageHeight = 0;

  late Image spellImage;
  bool spellImageLoaded = false;

  List<String> autocompletes = [];
  String base = "";

  String evalTree = "Loading...";

  RustApplication rustApplication = RustApplication();
  FocusNode focusNode = FocusNode();

  int numberOfSuggestions = 0;

  void _updateEvalTree() {
    rustApplication.fetchEvalTree(ansi: "false").then((value) => setState(() {
      evalTree = value;
    }));
  }

  void _updateImage(String expression) {
    rustApplication.evaluateExpression(expression: expression).then((value) {
        var (bytes, height) = value;
        setState(() {
          spellImage = Image.memory(bytes);
          imageHeight = height.toDouble();
          spellImageLoaded = true;
        });
    });
  }

  void _updateSuggestions() {
    rustApplication.autocompleteExpression().then((value) => setState(() {
      var (baseString, completions) = value;
      autocompletes = completions;
      base = baseString.isNotEmpty ? "$baseString ": "";
    }));
  }

  void _handleSubmitted(String value) {
    rustApplication.copyImage();
    focusNode.requestFocus();
  }

  void _handleChanged(String value) {
    _updateImage(value);
    _updateSuggestions();
    _updateEvalTree();
  }
  @override
  void dispose() {
    // Clean up the controller when the widget is removed from the
    // widget tree.
    super.dispose();
  }

  @override
  void initState() {
    super.initState();

    _handleChanged("");

    rustApplication.getNumberOfSuggestions().then((value) => setState(() {
      numberOfSuggestions = value;
    }));
  }

  autocompleteCommand(int key) {
    if (autocompletes.length > key) {
      textcontroller.text = "$base${autocompletes[key]} ";
      textcontroller.selection = TextSelection.fromPosition(TextPosition(offset: textcontroller.text.length));
      _updateImage(textcontroller.text);
      _updateSuggestions();
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: Center(
          child: CallbackShortcuts(
            bindings: <ShortcutActivator, VoidCallback>{
              const SingleActivator(LogicalKeyboardKey.digit1, control: true): () {
                autocompleteCommand(0);
              },
              const SingleActivator(LogicalKeyboardKey.digit2, control: true): () {
                autocompleteCommand(1);
              },
              const SingleActivator(LogicalKeyboardKey.digit3, control: true): () {
                autocompleteCommand(2);
              },
              const SingleActivator(LogicalKeyboardKey.digit4, control: true): () {
                autocompleteCommand(3);
              },
              const SingleActivator(LogicalKeyboardKey.digit5, control: true): () {
                autocompleteCommand(4);
              },
              const SingleActivator(LogicalKeyboardKey.digit6, control: true): () {
                autocompleteCommand(5);
              },
              const SingleActivator(LogicalKeyboardKey.digit7, control: true): () {
                autocompleteCommand(6);
              },
              const SingleActivator(LogicalKeyboardKey.digit8, control: true): () {
                autocompleteCommand(7);
              },
              const SingleActivator(LogicalKeyboardKey.digit9, control: true): () {
                autocompleteCommand(8);
              },
              const SingleActivator(LogicalKeyboardKey.digit0, control: true): () {
                autocompleteCommand(9);
              },
              const SingleActivator(LogicalKeyboardKey.escape): () {
                exit(0);
              },
              const SingleActivator(LogicalKeyboardKey.enter): () {
                _handleSubmitted(textcontroller.text);
              },
              const SingleActivator(LogicalKeyboardKey.enter, control: true): () {
                rustApplication.copyEvalTree();
              },
            },
            child: Column(
              children: [
                Container(
                  decoration: wrapper,
                  padding: const EdgeInsets.all(8),
                  child: Column(mainAxisSize: MainAxisSize.min,
                  children: [
                    for (var i = 0; i < numberOfSuggestions; i++)
                      Row(
                        children: [
                          Expanded(child: AutocompleteSuggestion(suggestion: i < autocompletes.length ? autocompletes[i] : "")),
                        ],
                      ),
                  ],
                  ),
                ),
                Container(
                  decoration: altWrapper,
                  padding: const EdgeInsets.all(4),
                  child: TextField(
                    autofocus: true,
                    focusNode: focusNode,
                    controller: textcontroller,
                    onChanged: (value) {
                      _handleChanged(value);
                    },
                    onSubmitted: _handleSubmitted,
                    style: const TextStyle(
                      color: Color.fromARGB(255, 255, 255, 255),
                      fontSize: 16,
                    ),
                  ),
                ),
                Container(
                  decoration: wrapper,
                  height: imageHeight.toDouble() + 16,
                  alignment: Alignment.topCenter,
                  padding: const EdgeInsets.all(8),
                  child: spellImageLoaded ? spellImage : const Text("Loading..."),
                ),
                Row(
                  children: [
                    Expanded(
                      child: Container(
                        decoration: altWrapper,
                        padding: const EdgeInsets.all(8),
                        child: const Text(
                          "Evaluation",
                          style: TextStyle(
                            color: Color.fromARGB(255, 255, 255, 255),
                            fontSize: 16,
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
                Expanded(
                  child: Container(
                    decoration: wrapper,
                    padding: const EdgeInsets.all(8),
                    child: SingleChildScrollView(
                      child: Text(
                        evalTree,
                        style: const TextStyle(
                          color: Color.fromARGB(255, 255, 255, 255),
                          fontSize: 16,
                          fontFamily: "Monospace",
                          height: 0,
                        ),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}