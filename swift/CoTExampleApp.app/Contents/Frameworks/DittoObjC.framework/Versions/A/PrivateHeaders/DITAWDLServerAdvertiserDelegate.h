//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

@protocol DITAWDLServerAdvertiser;
@protocol DITAWDLServerPeer;

NS_ASSUME_NONNULL_BEGIN

@protocol DITAWDLServerAdvertiserDelegate <NSObject>

- (NSNumber *)awdlServerAdvertiserAllocateID:(NSObject<DITAWDLServerAdvertiser> *)awdlServerAdvertiser;
- (void)awdlServerAdvertiser:(NSObject<DITAWDLServerAdvertiser> *)awdlServerAdvertiser addConnection:(NSObject<DITAWDLServerPeer> *)peer withID:(NSNumber *)peerID;
- (void)awdlServerAdvertiser:(NSObject<DITAWDLServerAdvertiser> *)awdlServerAdvertiser removeConnection:(NSNumber *)peerID;
- (void)awdlServerAdvertiser:(NSObject<DITAWDLServerAdvertiser> *)awdlServerAdvertiser performWithTransportHandle:(void (^)(void *handle))action;

@end

NS_ASSUME_NONNULL_END
