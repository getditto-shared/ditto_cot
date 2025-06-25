# Ditto Document Schema (deprecated in favor of JSON Schema)

## Overview

The Ditto database is organized into Collections which contains documents. This document provides a
schema for Ditto documents for the Ditto Edge Sync plugin for ATAK.

## Common Properties

Ditto document properties which are common across all document types (i.e. all collections).

| Name | Type    | Description                                  | Details                                     |
|------|---------|----------------------------------------------|---------------------------------------------|
| _id  | String  | Ditto document ID                            | Set based on collection type                |
| _c   | Counter | Document counter                             | Number of document updates, starting with 0 |
| _v   | Int     | Document schema version                      | Currently '2'                               |
| _r   | Boolean | Captures whether document has been deleted   | Soft-delete flag                            |
| a    | String  | Identity of Ditto peer of most recent update | Ditto Peer Key String                       |
| b    | Double  | Time of the most recent update               | Millis since epoch                          |
| d    | String  | UID of the author of most recent update      | TAK UID                                     |
| e    | String  | Callsign of the author of most recent update | TAK Callsign                                |

## CoT Properties

When reading a property from a ditto collection, if the value is absent or null, the default value
is used.

| Name | Type    | Description         | Default Value |
|------|---------|---------------------|---------------|
| g    | String  | Version             | Empty String  |
| h    | Double  | CotPoint CE         | 0.0           |
| i    | Double  | CotPoint HAE        | 0.0           |
| j    | Double  | CotPoint LAT        | 0.0           |
| k    | Double  | CotPoint LE         | 0.0           |
| l    | Double  | CotPoint LON        | 0.0           |
| n    | Long    | Start               | 0             |
| o    | Long    | Stale               | 0             |
| p    | String  | How                 | Empty String  |
| q    | String  | Access              | Empty String  |
| r    | String  | Detail (see note 1) | Empty String  |
| s    | String  | Opex                | Empty String  |
| t    | String  | Qos                 | Empty String  |
| u    | String  | Caveat              | Empty String  |
| v    | String  | Releasable to       | Empty String  |
| w    | String  | Type                | Empty String  |

_note 1_: This string MUST be the XML representation of a CotDetail.

## Collection: mapitem and track

These collections share a document schema
**mapitem** collection stores map items (map graphics) which are persistent

**track** collection stores PLI and location tracks which are transient

| Name                       | Type    | Description                          | Details                                |
|----------------------------|---------|--------------------------------------|----------------------------------------|
| _id                        | String  | CoT UID                              | Common Property (see above)            |
| c                          | String  | Name or title of the map item        | Typically item name or callsign        |
| f                          | Boolean | Captures whether document is visible | Map item may be hidden but not removed |
| Cot Properties (see above) |         |                                      |                                        |

## Collection: file

**file** collection stores images and other files

| Name        | Type   | Details          | Description                                      |
|-------------|--------|------------------|--------------------------------------------------|
| _id         | String | Hash of the file | Common Property (see above)                      |
| c           | String | File name        |                                                  |
| sz          | Double | File size        | Number of bytes                                  |
| file        | String | Attachment token | File contents                                    |
| mime        | String | MIME type        |                                                  |
| contentType | String | Content type     |                                                  |
| itemId      | String | ID of map item   | Only populated if file is attached to a map item |

## Collection: chat

**chat** collection stores chat messages

| Name             | Type   | Details                                  | Description            |
|------------------|--------|------------------------------------------|------------------------|
| _id              | String | Id of the message                        |                        |
| message          | String | The message                              |                        |
| room             | String | Room the message was sent to             |                        |
| parent           | String |                                          |                        |
| roomId           | String | id of the room                           |                        |
| authorCallsign   | String | Callsign of the sender                   |                        |
| authorUid        | String | Uid of the sender                        |                        |
| authorType       | String |                                          |                        |
| time             | String | time message was sent                    |                        |
| location         | String | location of sender when message was sent | A GeoPoint as a String |
| Cot Properties ^ |        |                                          |                        |

## Collection: alert

**alert** collection stores alerts

| Name | Type   | Details | Description |
|------|--------|---------|-------------|
| _id  | String |         |             |
|      |        |         |             |

## Collection: api

**api** collection stores documents for Ditto plugin API e.g. documents from other plugins

| Name        | Type    | Details                                                | Description |
|-------------|---------|--------------------------------------------------------|-------------|
| _id         | String  | id of the document                                     |             |
| isFile      | Boolean | Whether this document is a file                        |             |
| title       | String  | Title of document                                      |             |
| mime        | String  | file type as MIME                                      |             |
| contentType | String  |                                                        |             |
| tag         | String  |                                                        |             |
| data        | String  | data of the document                                   |             |
| isRemoved   | Boolean | whether this document still exists on the local device |             |
| timeMillis  | Long    | Time of document creation                              |             |

## Sample track/PLI document (version 2)
```
{
  "_c": 14365,
  "_id": "ANDROID-6d2198a6271bca69",
  "_r": false,
  "_v": 2,
  "a": "pkAocCgkMDQ1_BWQXXkjEah7pV_2rvS4TTwwkJ6qeUpBPRYrAlphs",
  "b": 1748370358459,
  "c": "GATOR",
  "d": "ANDROID-6d2198a6271bca69",
  "e": "GATOR",
  "f": true,
  "g": "2.0",
  "h": 27.5,
  "i": -30.741204952759624,
  "j": 27.020123,
  "k": 9999999,
  "l": -81.261311,
  "n": 1748370358459,
  "o": 1748370433459,
  "p": "m-g",
  "q": "Undefined",
  "r": "<detail><takv os='34' version='4.10.0.57 (e9ad8ffb).1724408135-CIV' device='GOOGLE PIXEL 8A' platform='ATAK-CIV'/><contact endpoint='192.168.1.116:4242:tcp' callsign='GATOR'/><uid Droid='GATOR'/><precisionlocation altsrc='GPS' geopointsrc='GPS'/><__group role='Team Member' name='Cyan'/><status battery='14'/><ditto a='pkAocCgkMDQ1_BWQXXkjEah7pV_2rvS4TTwwkJ6qeUpBPRYrAlphs' ip='192.168.1.116' version='AndJ4.10.2_90aa996a2e' deviceName='T71bca69'/></detail>",
  "r1": "0.0",
  "r2": "123.1634706584482",
  "w": "a-f-G-U-C"
}
```

## Sample map item document (version 2)
```
{
  "_c": 0,
  "_id": "4fb3351d-01c6-46c1-adab-185c85e3acaf",
  "_r": false,
  "_v": 2,
  "a": "pkAocCgkMD0CRcjfRZhyUiTRi7IRgdD0ZW9vtQekaZwgx9yITCEzA",
  "b": 1747692836124,
  "c": "TOUCH.19.181356",
  "d": "ANDROID-676ec9f9ab979bdd",
  "e": "TOUCH",
  "f": true,
  "g": "2.0",
  "h": 9999999,
  "i": -52.341097852638754,
  "j": 25.020064899999998,
  "k": 9999999,
  "l": -81.2613527,
  "n": 1747692836124,
  "o": 1779228836124,
  "p": "h-g-i-g-o",
  "q": "Undefined",
  "r": "<detail><status readiness='true'/><archive/><remarks></remarks><creator uid='ANDROID-676ec9f9ab979bdd' callsign='TOUCH' time='2025-05-19T22:13:56.119Z' type='a-f-G-U-C'/><contact callsign='TOUCH.19.181356'/><archive/><color argb='-1'/><link uid='ANDROID-676ec9f9ab979bdd' production_time='2025-05-19T22:13:56.119Z' type='a-f-G-U-C' parent_callsign='TOUCH' relation='p-p'/></detail>",
  "w": "b-i-x-i"
}
```

## Upgrade from version 1

If you have previously integrated with version 1 document schema which was used during Beta testing for the Ditto Edge Sync plugin for ATAK, then note the following changes when migrating to version 2, which is used in the v1.0 of the Ditto Edge Sync plugin for ATAK: 
* The biggest change is the move from use of a Ditto map/json in v1 to the use of a Ditto string in 
v2 to store CoT detail. See property 'r'
* Note that the property short names have changed, as documented above
* Properties which are common to all collections and managed by Ditto are prefixed with '_' (e.g., '_v') 
* The version schema has changed from 1 to 2. See property '_v'
* v2 adds a Ditto counter to track the number of changes to a document. See property '_c'
