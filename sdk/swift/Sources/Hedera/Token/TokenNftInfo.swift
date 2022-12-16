/*
 * ‌
 * Hedera Swift SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

import CHedera
import Foundation

/// Response from `TokenNftInfoQuery`.
public final class TokenNftInfo: Codable {
    /// The ID of the NFT.
    public let nftId: NftId

    /// The current owner of the NFT.
    public let accountId: AccountId

    /// Effective consensus timestamp at which the NFT was minted.
    public let creationTime: Timestamp

    /// The unique metadata of the NFT.
    public let metadata: Data

    /// If an allowance is granted for the NFT, its corresponding spender account.
    public let spenderId: AccountId?

    /// The ledger ID the response was returned from
    public let ledgerId: LedgerId

    public static func fromBytes(_ bytes: Data) throws -> Self {
        try Self.fromJsonBytes(bytes)
    }

    public func toBytes() -> Data {
        // can't have `throws` because that's the wrong function signature.
        // swiftlint:disable force_try
        try! self.toJsonBytes()
    }
}

extension TokenNftInfo: ToFromJsonBytes {
    internal static var cToBytes: ToJsonBytesFunc { hedera_token_nft_info_to_bytes }

    internal static var cFromBytes: FromJsonBytesFunc { hedera_token_nft_info_from_bytes }
}
