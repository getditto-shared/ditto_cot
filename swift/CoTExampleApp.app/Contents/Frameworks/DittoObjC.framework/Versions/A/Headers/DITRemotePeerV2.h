//
// Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@class DITAddress;

/**
 Represents a remote peer in the Ditto network.
 */
@interface DITRemotePeerV2 : NSObject

/**
 Uniquely identifies a peer within a Ditto mesh network.
 */
@property (nonatomic, readonly) DITAddress *address;

/**
 Advertising identifier of the remote peer.
 */
@property (nonatomic, readonly) uint32_t networkID;

/**
 The human-readable device name of the remote peer. This defaults to the hostname
 but can be manually set by the application developer of the other peer.
 It is not necessarily unique.
 */
@property (nonatomic, readonly, strong) NSString *deviceName;

/**
 Operating system of the remote peer.
 */
@property (nonatomic, readonly, strong) NSString *os;

/**
 An optional Query Overlap Group which can be assigned to group certain
 types of peers together and configure relative connection priorities.
 Defaults to 0 if not set.
 */
@property (nonatomic, readonly) UInt8 queryOverlapGroup __deprecated_msg(
    "Query overlap groups have been phased out, this property always returns 0.");

- (instancetype)init NS_UNAVAILABLE;

@end

NS_ASSUME_NONNULL_END
