//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

// IMPORTANT: DittoObjC is deprecated and moving to Swift-only. This type and
// its friends are only here to aid the Swift implementation and doesn't work
// on its own. Therefore, we're keeping it undocumented.

@interface DITStoreObserver : NSObject

@property (nonatomic, readonly, copy) NSString *queryString;
@property (nonatomic, readonly, nullable) NSData *queryArgsData;
@property (nonatomic, readonly, getter=isStopped) BOOL stopped;

- (instancetype)init NS_UNAVAILABLE;
- (void)stop;

@end

NS_ASSUME_NONNULL_END
