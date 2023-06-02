import 'package:wasm_wit_component/src/component.dart';

/// A Rust-style Result type.
sealed class Result<O, E> {
  const factory Result.ok(O ok) = Ok<O, E>;
  const factory Result.err(E error) = Err<O, E>;

  /// Returns `true` if this is an [Ok] instance
  bool get isOk;

  /// Returns `true` if this is an [Err] instance
  bool get isError;

  /// Returns the contained [O] ok value, if present, otherwise returns null
  O? get ok;

  /// Returns the contained [E] error value, if present, otherwise returns null
  E? get error;

  factory Result.fromJson(
    Object? json,
    O Function(Object? json) ok,
    E Function(Object? json) err,
  ) {
    return switch (json) {
      {'ok': final o} || (0, final o) => Ok(ok(o)),
      {'error': final e} || (1, final e) => Err(err(e)),
      _ => throw Exception('Invalid JSON for Result: $json'),
    };
  }

  Map<String, Object?> toJson([
    Object? Function(O value)? mapOk,
    Object? Function(E value)? mapError,
  ]);
}

/// A Rust-style Result type's success value.
class Ok<O, E> implements Result<O, E> {
  @override
  final O ok;
  const Ok(this.ok);

  @override
  bool get isOk => true;
  @override
  bool get isError => false;
  @override
  E? get error => null;

  @override
  Map<String, Object?> toJson([
    Object? Function(O value)? mapOk,
    Object? Function(E value)? mapError,
  ]) =>
      {'ok': mapOk == null ? ok : mapOk(ok)};

  @override
  bool operator ==(Object other) =>
      other is Ok<O, E> &&
      other.runtimeType == runtimeType &&
      comparator.areEqual(ok, other.ok);

  @override
  int get hashCode => runtimeType.hashCode ^ comparator.hashValue(ok);

  @override
  String toString() {
    return 'Ok<$O, $E>($ok)';
  }
}

/// A Rust-style Result type's failure value.
class Err<O, E> implements Result<O, E> {
  @override
  final E error;
  const Err(this.error);

  @override
  bool get isOk => false;
  @override
  bool get isError => true;
  @override
  O? get ok => null;

  @override
  Map<String, Object?> toJson([
    Object? Function(O value)? mapOk,
    Object? Function(E value)? mapError,
  ]) =>
      {'error': mapError == null ? error : mapError(error)};

  @override
  bool operator ==(Object other) =>
      other is Err<O, E> &&
      other.runtimeType == runtimeType &&
      comparator.areEqual(error, other.error);

  @override
  int get hashCode => runtimeType.hashCode ^ comparator.hashValue(error);

  @override
  String toString() {
    return 'Err<$O, $E>($error)';
  }
}