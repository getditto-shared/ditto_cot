//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITTransportConfig.h>

NS_ASSUME_NONNULL_BEGIN

NSDictionary *DITTransportConfigAsDictionary(DITTransportConfig *transportConfig);
DITTransportConfig *DITTransportConfigFromDictionary(NSDictionary *dictionary);

@interface DITTransportConfig () {
    @protected
    DITPeerToPeer *_peerToPeer;
    @protected
    DITConnect *_connect;
    @protected
    DITListen *_listen;
    @protected
    DITGlobalConfig *_global;
}

- (instancetype)initWithPeerToPeer:(DITPeerToPeer *)peerToPeer
                           connect:(DITConnect *)connect
                            listen:(DITListen *)listen
                            global:(DITGlobalConfig *)global
                              copy:(BOOL)shouldCopy NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
