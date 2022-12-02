// DO NOT EDIT.
// swift-format-ignore-file
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: token_grant_kyc.proto
//
// For information on using the generated types, please see the documentation:
//   https://github.com/apple/swift-protobuf/

import Foundation
import SwiftProtobuf

// If the compiler emits an error on this type, it is because this file
// was generated by a version of the `protoc` Swift plug-in that is
// incompatible with the version of SwiftProtobuf to which you are linking.
// Please ensure that you are building against the same version of the API
// that was used to generate this file.
fileprivate struct _GeneratedWithProtocGenSwiftVersion: SwiftProtobuf.ProtobufAPIVersionCheck {
  struct _2: SwiftProtobuf.ProtobufAPIVersion_2 {}
  typealias Version = _2
}

///*
/// Grants KYC to the account for the given token. Must be signed by the Token's kycKey.
/// If the provided account is not found, the transaction will resolve to INVALID_ACCOUNT_ID.
/// If the provided account has been deleted, the transaction will resolve to ACCOUNT_DELETED.
/// If the provided token is not found, the transaction will resolve to INVALID_TOKEN_ID.
/// If the provided token has been deleted, the transaction will resolve to TOKEN_WAS_DELETED.
/// If an Association between the provided token and account is not found, the transaction will
/// resolve to TOKEN_NOT_ASSOCIATED_TO_ACCOUNT.
/// If no KYC Key is defined, the transaction will resolve to TOKEN_HAS_NO_KYC_KEY.
/// Once executed the Account is marked as KYC Granted.
public struct Proto_TokenGrantKycTransactionBody {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  ///*
  /// The token for which this account will be granted KYC. If token does not exist, transaction
  /// results in INVALID_TOKEN_ID
  public var token: Proto_TokenID {
    get {return _token ?? Proto_TokenID()}
    set {_token = newValue}
  }
  /// Returns true if `token` has been explicitly set.
  public var hasToken: Bool {return self._token != nil}
  /// Clears the value of `token`. Subsequent reads from it will return its default value.
  public mutating func clearToken() {self._token = nil}

  ///*
  /// The account to be KYCed
  public var account: Proto_AccountID {
    get {return _account ?? Proto_AccountID()}
    set {_account = newValue}
  }
  /// Returns true if `account` has been explicitly set.
  public var hasAccount: Bool {return self._account != nil}
  /// Clears the value of `account`. Subsequent reads from it will return its default value.
  public mutating func clearAccount() {self._account = nil}

  public var unknownFields = SwiftProtobuf.UnknownStorage()

  public init() {}

  fileprivate var _token: Proto_TokenID? = nil
  fileprivate var _account: Proto_AccountID? = nil
}

#if swift(>=5.5) && canImport(_Concurrency)
extension Proto_TokenGrantKycTransactionBody: @unchecked Sendable {}
#endif  // swift(>=5.5) && canImport(_Concurrency)

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "proto"

extension Proto_TokenGrantKycTransactionBody: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  public static let protoMessageName: String = _protobuf_package + ".TokenGrantKycTransactionBody"
  public static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .same(proto: "token"),
    2: .same(proto: "account"),
  ]

  public mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularMessageField(value: &self._token) }()
      case 2: try { try decoder.decodeSingularMessageField(value: &self._account) }()
      default: break
      }
    }
  }

  public func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    // The use of inline closures is to circumvent an issue where the compiler
    // allocates stack space for every if/case branch local when no optimizations
    // are enabled. https://github.com/apple/swift-protobuf/issues/1034 and
    // https://github.com/apple/swift-protobuf/issues/1182
    try { if let v = self._token {
      try visitor.visitSingularMessageField(value: v, fieldNumber: 1)
    } }()
    try { if let v = self._account {
      try visitor.visitSingularMessageField(value: v, fieldNumber: 2)
    } }()
    try unknownFields.traverse(visitor: &visitor)
  }

  public static func ==(lhs: Proto_TokenGrantKycTransactionBody, rhs: Proto_TokenGrantKycTransactionBody) -> Bool {
    if lhs._token != rhs._token {return false}
    if lhs._account != rhs._account {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}
