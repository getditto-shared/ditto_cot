//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITConnect.h>
#import <DittoObjC/DITListen.h>
#import <DittoObjC/DITPeerToPeer.h>
#import <DittoObjC/DITGlobalConfig.h>
#import <DittoObjC/DITConnect.h>

#import <Foundation/Foundation.h>

@class DITMutableTransportConfig;

NS_ASSUME_NONNULL_BEGIN

/**
 A configuration object specifying which network transports Ditto should use to sync data.

 A DITDitto object comes with a default transport configuration where all available peer-to-peer
 transports are enabled. You can customize this by initializing a DITMutableTransportConfig,
 adjusting its properties, and supplying it to setTransportConfig: on DITDitto.
 When you initialize a DITMutableTransportConfig yourself it starts with all transports disabled.
 You must enable each one directly.
 */

@interface DITTransportConfig : NSObject <NSCopying, NSMutableCopying>

/**
 TransportConfig instance type.
 */
+ (instancetype)transportConfig;

/**
 Peer-to-peer transports will automatically discover peers in the vicinity and create connections
 without any configuration. These are configured inside the peerToPeer property. To turn each
 one on, set its enabled property to YES.
 */
@property (nonatomic, readonly) DITPeerToPeer *peerToPeer;

/**
 To connect to a peer at a known location, such as a Ditto Big Peer, add its address inside
 the connect configuration. These are either "host:port" strings for raw TCP sync, or a "wss://…"
 URL for websockets.
 */
@property (nonatomic, readonly) DITConnect *connect;

/**
 The listen configurations are for specific less common data sync scenarios. Please read the
 documentation on the Ditto website for examples. Incorrect use of listen can result in
 insecure configurations.
 */
@property (nonatomic, readonly) DITListen *listen;

/**
 The global configuration is transport agnostic.
 */
@property (nonatomic, readonly) DITGlobalConfig *global;

/**
 Initialize with DITTransportConfig.
 */
- (instancetype)initWithDITTransportConfig:(DITTransportConfig *)config;

/**
 Initialize DITTransportConfig with configuration subobjects.
 */
- (instancetype)initWithPeerToPeer:(DITPeerToPeer *)peerToPeer
                           connect:(DITConnect *)connect
                            listen:(DITListen *)listen
                            global:(DITGlobalConfig *)global;

@end

// MARK: -

@interface DITTransportConfig (DITTypeCorrections)

- (DITTransportConfig *)copy;
- (DITMutableTransportConfig *)mutableCopy;

@end

NS_ASSUME_NONNULL_END
