<?xml version="1.0" encoding="UTF-8"?>
<tileset name="basic_tileset" tilewidth="64" tileheight="64" tilecount="540" columns="27">
 <image source="tilesheet_complete.png" width="1728" height="1280"/>
 <terraintypes>
  <terrain name="grass" tile="-1"/>
  <terrain name="dirt" tile="-1"/>
  <terrain name="wooden floor" tile="41"/>
 </terraintypes>
 <tile id="0" terrain="0,0,0,0">
  <objectgroup draworder="index"/>
 </tile>
 <tile id="1" terrain="0,0,0,0"/>
 <tile id="2" terrain="0,0,0,0"/>
 <tile id="3" terrain="0,0,0,0"/>
 <tile id="4" terrain="1,1,1,1"/>
 <tile id="5" terrain="1,1,1,1"/>
 <tile id="41" terrain="2,2,2,2">
  <objectgroup draworder="index" name="wooden_floor_1">
   <properties>
    <property name="movement_cost" type="int" value="1"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="42" terrain="2,2,2,2">
  <objectgroup draworder="index" name="wooden_floor_2">
   <properties>
    <property name="movement_cost" type="int" value="1"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="43" terrain="2,2,2,2">
  <objectgroup draworder="index" name="wooden_floor_3">
   <properties>
    <property name="movement_cost" type="int" value="1"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="44" terrain="2,2,2,2">
  <objectgroup draworder="index" name="wooden_floor_4">
   <properties>
    <property name="movement_cost" type="int" value="1"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="45" terrain="2,2,2,2">
  <objectgroup draworder="index" name="wooden_floor_5">
   <properties>
    <property name="movement_cost" type="int" value="1"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="46" terrain="2,2,2,2">
  <objectgroup draworder="index" name="wooden_floor_6">
   <properties>
    <property name="movement_cost" type="int" value="1"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="441">
  <objectgroup draworder="index">
   <properties>
    <property name="movement_cost" type="int" value="2"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="445">
  <objectgroup draworder="index">
   <properties>
    <property name="movement_cost" type="int" value="2"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="492">
  <objectgroup draworder="index">
   <properties>
    <property name="blocks_vision" type="bool" value="true"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="493">
  <objectgroup draworder="index">
   <properties>
    <property name="blocks_vision" type="bool" value="true"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="519">
  <objectgroup draworder="index">
   <properties>
    <property name="blocks_vision" type="bool" value="false"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="520">
  <objectgroup draworder="index">
   <properties>
    <property name="blocks_vision" type="bool" value="true"/>
   </properties>
  </objectgroup>
 </tile>
 <tile id="539">
  <objectgroup draworder="index">
   <properties>
    <property name="movement_cost" type="int" value="-1"/>
   </properties>
  </objectgroup>
 </tile>
</tileset>
