import 'package:flutter/material.dart';
import 'package:flutter_example/flutter_utils.dart';

class CodeTextField extends StatelessWidget {
  const CodeTextField({
    super.key,
    required this.controller,
    required this.scrollController,
    this.focusNode,
    this.lineWidgets,
  });

  final TextEditingController controller;
  final ScrollController scrollController;
  final FocusNode? focusNode;
  final Map<int, Widget>? lineWidgets;

  static const lineHeight = 19.0;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, box) {
        final lines = (box.maxHeight / lineHeight).floor();
        return Row(
          children: [
            if (lineWidgets != null)
              AnimatedBuilder(
                animation: scrollController,
                builder: (context, _) {
                  double bottomPad = 0;
                  int initial =
                      scrollController.initialScrollOffset ~/ lineHeight + 1;
                  int delta = 1;
                  try {
                    initial = scrollController.offset ~/ lineHeight + 1;
                    bottomPad = scrollController.offset % lineHeight;
                    if (scrollController.offset <= 0) {
                      initial = 0;
                      bottomPad = 0;
                      delta = 0;
                    }
                  } catch (_) {}

                  return SizedBox(
                    // width: 50,
                    height: lineHeight * lines + 4,
                    child: Column(
                      children: [
                        const SizedBox(height: 4),
                        if (delta != 0)
                          SizedBox(height: lineHeight - bottomPad),
                        for (var i = initial; i < lines + initial - delta; i++)
                          SizedBox(
                            height: lineHeight,
                            child: lineWidgets?[i] ?? Text('${i + 1}'),
                          ),
                        SizedBox(height: bottomPad),
                      ],
                    ),
                  );
                },
              ),
            Expanded(
              child: SingleChildScrollView(
                scrollDirection: Axis.horizontal,
                child: controller.select(
                  (state) => state.text,
                  (context, text) {
                    final list = text.split('\n');
                    final maxCharacters = list.isEmpty
                        ? 100
                        : list
                            .reduce(
                              (value, element) => value.length > element.length
                                  ? value
                                  : element,
                            )
                            .length;
                    final w = maxCharacters * 8.3 + 24.0;
                    return SizedBox(
                      width: box.maxWidth > w ? box.maxWidth : w,
                      child: TextField(
                        style: codeTextStyle,
                        scrollController: scrollController,
                        focusNode: focusNode,
                        controller: controller,
                        expands: true,
                        maxLines: null,
                        minLines: null,
                      ),
                    );
                  },
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}
