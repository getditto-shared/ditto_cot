//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITPeerToPeer.h>

#import <DittoObjC/DITMutableAWDLConfig.h>
#import <DittoObjC/DITMutableBluetoothLEConfig.h>
#import <DittoObjC/DITMutableLANConfig.h>

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN
/*
 Configuration of peer-to-peer transports, which are able to discover and connect to
 peers on their own.

 Contains a configuration for each type of peer-to-peer transport.

 For more information refer to the documentation for `DITTransportConfig`.
 */
@interface DITMutablePeerToPeer : DITPeerToPeer

@property (nonatomic, readonly, copy) DITMutableBluetoothLEConfig *bluetoothLe;
@property (nonatomic, readonly, copy) DITMutableLANConfig *lan;
@property (nonatomic, readonly, copy) DITMutableAWDLConfig *awdl;

- (void)setBluetoothLe:(DITBluetoothLEConfig *)bluetoothLe;
- (void)setLan:(DITLANConfig *)lan;
- (void)setAwdl:(DITAWDLConfig *)awdl;

@end

NS_ASSUME_NONNULL_END
