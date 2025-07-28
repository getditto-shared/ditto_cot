//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITConnect.h>

NS_ASSUME_NONNULL_BEGIN

@interface DITConnect () {
    @protected
    NSSet<NSString *> *_tcpServers;
    @protected
    NSSet<NSString *> *_websocketURLs;
    @protected
    NSTimeInterval _retryInterval;
}

- (instancetype)initWithTCPServers:(NSSet<NSString *> *)tcpServers
                     websocketURLs:(NSSet<NSString *> *)websocketUrls
                     retryInterval:(NSTimeInterval)retryInterval
                              copy:(BOOL)shouldCopy NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
