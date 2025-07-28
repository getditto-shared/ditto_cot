//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITConnect.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/**
 Mutable DITConnect interface to configure specific servers Ditto should attempt to connect to.
 They are either TCP or WebSocket servers. TCP servers take `host:port` syntax and WebSocket
 URLs take the form `wss://server.example.com`.

 Please refer to the documentation on Ditto's website for configuring cloud or client/server
scenarios.
 */

@interface DITMutableConnect : DITConnect

@property (nonatomic, readonly) NSMutableSet<NSString *> *tcpServers;
@property (nonatomic, readonly) NSMutableSet<NSString *> *websocketURLs;
@property (nonatomic, readwrite) NSTimeInterval retryInterval;

- (void)setTcpServers:(NSSet<NSString *> *)tcpServers;
- (void)setWebsocketURLs:(NSSet<NSString *> *)websocketURLs;

@end

NS_ASSUME_NONNULL_END
