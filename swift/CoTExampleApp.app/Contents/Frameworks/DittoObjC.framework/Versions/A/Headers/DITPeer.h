//
// Copyright © 2022 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

@class DITAddress;
@class DITConnection;

NS_ASSUME_NONNULL_BEGIN

/** An instance of Ditto taking part in the mesh network. */
@interface DITPeer : NSObject <NSCopying>

/**
 Uniquely identifies a peer within a Ditto mesh network.
 */
@property (nonatomic, readonly, copy) DITAddress *address;

/**
 * The peer key is a unique identifier for a given peer, equal to or derived
 * from the cryptographic public key used to authenticate it.
 *
 * NOTE: This will be be empty when a peer is not updated to the latest
 * version of the SDK.
 */
@property (nonatomic, readonly, copy) NSData *peerKey;

/**
 Currently active connections of the peer.
 */
@property (nonatomic, readonly, copy) NSArray<DITConnection *> *connections;

/**
 The human-readable device name of the peer. This defaults to the hostname
 but can be manually set by the application developer of the other peer.
 It is not necessarily unique.
 */
@property (nonatomic, readonly, copy) NSString *deviceName;

/** The operating system the peer is running on, `nil` if (yet) unknown. */
// REFACTOR: make this a proper enum mirroring `PresenceOs` from the Core.
@property (nonatomic, readonly, nullable, copy) NSString *os;

/** The Ditto SDK version the peer is running with, `nil` if (yet) unknown. */
@property (nonatomic, readonly, nullable, copy) NSString *dittoSDKVersion;

/** Indicates whether the peer is connected to Ditto Cloud. */
@property (nonatomic, readonly, getter=isConnectedToDittoCloud) BOOL connectedToDittoCloud;

/** An `NSNumber` containing a Boolean Indicating whether the peer is compatible with the current
 * peer, `nil` if (yet) unknown. */
@property (nonatomic, readonly, nullable, copy) NSNumber *isCompatible;

/**
 An optional Query Overlap Group which can be assigned to group certain
 types of peers together and configure relative connection priorities.
 Defaults to 0 if not set.
 */
@property (nonatomic, readonly) UInt8 queryOverlapGroup __deprecated_msg(
    "Query overlap groups have been phased out, this property always returns 0.");

/**
 Convenience initializer, initializes the peer with the passed in
 dictionary representation. Raises an `NSInvalidArgumentException` if the
 dictionary representation is not a valid peer representation.
 */
// NOTE: exception instead of `NSError` here on purpose, passing an invalid dictionary
// representation is considered a programming error.
- (instancetype)initWithDictionaryRepresentation:(NSDictionary *)dictionaryRepresentation;

/**
 Initializes the peer with all possible parameters.
 */
- (instancetype)initWithAddress:(DITAddress *)address
                        peerKey:(NSData *)peerKey
                    connections:(NSArray<DITConnection *> *)connections
                     deviceName:(NSString *)deviceName
                             os:(nullable NSString *)os
                dittoSDKVersion:(nullable NSString *)dittoSDKVersion
        isConnectedToDittoCloud:(BOOL)isConnectedToDittoCloud
                   isCompatible:(nullable NSNumber *)isCompatible
              queryOverlapGroup:(UInt8)queryOverlapGroup NS_DESIGNATED_INITIALIZER;
- (instancetype)init NS_UNAVAILABLE;

- (NSUInteger)hash;

- (BOOL)isEqual:(nullable id)object;
- (BOOL)isEqualToPeer:(DITPeer *)peer;

- (DITPeer *)copy;
- (DITPeer *)copyWithZone:(nullable NSZone *)zone;

@end

NS_ASSUME_NONNULL_END
